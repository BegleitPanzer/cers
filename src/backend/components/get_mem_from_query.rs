
use std::{iter::successors, sync::Arc};

use futures::{stream::FuturesUnordered, StreamExt};
use tokio::sync::Mutex;
use winapi::um::winnt;
use std::time::Instant;

use crate::{backend::process::process::Process, ui::main::{AMApp, App, Data}};

#[derive(Clone, Copy)]
struct WrappedMBI(winnt::MEMORY_BASIC_INFORMATION);
unsafe impl Send for WrappedMBI {}
unsafe impl Sync for WrappedMBI {}

#[derive(Debug, Copy, Clone)]
pub enum QueryTypes {
    Bytes2,
    Bytes4,
    Bytes8,
    Float,
    String
}

///
/// Scan's the selected processes memory for a specific value
/// This needs to be HEAVILY optimized. Cheat Engine is lightning fast, this isn't.
/// 
pub async fn get_mem_from_query( upper_bound: usize, lower_bound: usize, app: AMApp) {
    let start = Instant::now();
    if app.get_querying().await { return; }
    let pid = app.get_process().await as u32;
    let Ok(proc) = Process::open(pid)
    else { return; };
    let proc = proc;
    app.modify_progress_msg(format!("Scanning...")).await;
    app.modify_mem_view_list("reset", None).await;
    app.modify_querying(true).await;
    // filter memory regions by upper and lower bounds
    let mem = proc
        .memory_regions().
        into_iter().
        filter(|p| {
            ((p.Protect & winnt::PAGE_READWRITE) != 0)
            && ((p.BaseAddress as usize) >= lower_bound && (p.BaseAddress as usize) <= upper_bound)
        });
    
    let regions: Vec<WrappedMBI> = mem.map(|p| WrappedMBI(p)).collect::<Vec<_>>();

    let query = app.get_query().await.1; // don't want to lock this a million times
    // discern the type of the query for optimization sake
    let query_type: QueryTypes = {
        if query.parse::<u16>().is_ok() { QueryTypes::Bytes2 }
        else if query.parse::<u32>().is_ok() { QueryTypes::Bytes4 }
        else if query.parse::<u64>().is_ok() { QueryTypes::Bytes8 }
        else if query.parse::<f32>().is_ok() { QueryTypes::Float }
        else { QueryTypes::String }
    };


    app.modify_progress_msg(format!("Identified query as type {:#?}.", query_type)).await;
    app.modify_progress_msg(format!("Querying {} regions for {}...", regions.len(), query)).await;
    let mut query_results: Vec<usize> = vec![];
   
    // this makes N tasks where N = the number of regions.
    // Theoretically could be sped up even more by making tasks for each address in the region.
    // I wonder what the limit is?
    let task_count = regions.len();

    let mut tasks_finished = 0;
    let each_len = regions.len() / task_count + if regions.len() % task_count == 0 { 0 } else { 1 };
    let mut out = vec![Vec::with_capacity(each_len); task_count];
    for (i, d) in regions.iter().copied().enumerate() {
        out[i % task_count].push(d);
    }
    let mut futures = FuturesUnordered::new();
    for (idx, region) in out.into_iter().enumerate() {
        futures.push(spawn_mem_read_task(region, pid, query.clone(), query_type));
    }
    while let Some(result) = futures.next().await {
        tasks_finished += 1;
        query_results.extend(result);
        app.modify_query_progress(tasks_finished as f64 / task_count as f64).await;
        app.modify_progress_msg(format!("Task {}/{} finished", tasks_finished, task_count)).await;
    }
    app.modify_progress_msg(format!("Query complete.")).await;
    app.modify_query_progress(0.00).await;
    app.modify_query_results(query_results).await;
    app.modify_querying(false).await;
    app.modify_progress_msg(format!("Query took {} seconds.", start.elapsed().as_secs_f32())).await;
    
}

///
/// Spawns a task for reading a number of memory regions.
/// 
async fn spawn_mem_read_task(regions: Vec<WrappedMBI>, pid: u32, query: String, qtype: QueryTypes) -> Vec<usize> {
    let query_results: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![]));
    let proc = Process::open(pid).unwrap();
    tokio::spawn({let query_results = query_results.clone(); async move {
        match qtype {
            QueryTypes::Bytes2 | QueryTypes::Bytes4 | QueryTypes::Bytes8 => {
                let query = query.parse::<u32>().unwrap();
                for region in regions.clone().into_iter() {
                    let Ok(memory) = proc.read_memory(region.0.BaseAddress as _, region.0.RegionSize)
                    else { continue };
                    for (offset, _)  in memory.windows((query.ilog10() + 1) as usize).enumerate().into_iter().step_by((query.ilog10() + 1) as usize) {
                        let addr = region.0.BaseAddress as usize + offset as usize;
                        let Ok(value) = proc.value_at(addr)
                        else { continue; };
                        if value != query as u32 { continue; }
                        //app.modify_progress_msg(format!("Found value at address {:#X}.",addr)).await;
                        query_results.lock().await.push(addr);
                    }
                }
            }
            QueryTypes::String => {
                for region in regions.clone().into_iter() {
                    let Ok(memory) = proc.read_memory(region.0.BaseAddress as _, region.0.RegionSize)
                    else { continue; };
                    for (offset, window) in memory.windows(query.len()).enumerate() {
                        let addr = region.0.BaseAddress as usize + offset as usize;
                        let Ok(value) = String::from_utf8(window.to_vec())
                        else { continue; };
                        if value != query { continue; };
                        //app.modify_progress_msg(format!("Found value at address {:#X}.", addr)).await;
                        query_results.lock().await.push(addr);
                    }
                }
            }
            QueryTypes::Float => {
                for region in regions.clone().into_iter() {
                    let query = query.parse::<f32>().unwrap().to_le_bytes(); // technically doesn't always have to be a f64
                    let Ok(memory) = proc.read_memory(region.0.BaseAddress as _, region.0.RegionSize)
                    else { continue };
                    for (offset, _)  in memory.windows(query.len()).enumerate().into_iter().step_by((query.len()) as usize) {
                        let Ok(value) = proc.value_at(region.0.BaseAddress as usize + offset as usize)
                        else { continue };
                        let value = value.to_le_bytes();
                        if value != query { continue };
                        let addr = region.0.BaseAddress as usize + offset as usize;
                        query_results.lock().await.push(addr);
                    }
                }
            }
            _ => {} 
        }
    }}).await.unwrap();
    let r = query_results.lock().await.clone();
    r
}
