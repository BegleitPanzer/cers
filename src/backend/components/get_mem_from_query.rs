
use std::iter::successors;

use winapi::um::winnt;

use crate::{backend::process::process::Process, ui::main::{AMApp, App, Data}};

#[derive(Clone)]
struct WrappedMBI(winnt::MEMORY_BASIC_INFORMATION);
unsafe impl Send for WrappedMBI {}

#[derive(Debug)]
pub enum QueryTypes {
    Bytes2,
    Bytes4,
    Bytes8,
    Float,
    Double,
    String
}

pub async fn get_mem_from_query( upper_bound: usize, lower_bound: usize, app: AMApp ) {
    if app.get_querying().await { return; }
    let Ok(proc) = Process::open(app.get_process().await as u32)
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
    // discern the type of the query
    let query_type: QueryTypes = {
        if query.parse::<u16>().is_ok() { QueryTypes::Bytes2 }
        else if query.parse::<u32>().is_ok() { QueryTypes::Bytes4 }
        else if query.parse::<u64>().is_ok() { QueryTypes::Bytes8 }
        else if query.parse::<f32>().is_ok() { QueryTypes::Float }
        else if query.parse::<f64>().is_ok() { QueryTypes::Double }
        else { QueryTypes::String }
    };

    app.modify_progress_msg(format!("Identified query as type {:#?}.", query_type)).await;
    
    match query_type {
        QueryTypes::Bytes2 | QueryTypes::Bytes4 | QueryTypes::Bytes8  => {
            app.modify_progress_msg(format!("Querying {} regions for {}...", regions.len(), query)).await;
            for region in regions.clone().into_iter() {
                let query = query.parse::<u32>().unwrap();
                app.modify_query_progress(regions.clone().into_iter().position(|p| p.0.BaseAddress == region.0.BaseAddress).unwrap() as f64 / regions.len() as f64).await;
                let Ok(memory) = proc.read_memory(region.0.BaseAddress as _, region.0.RegionSize)
                else { continue; };
                for (offset, window) in memory.windows((query.ilog10() + 1) as usize).enumerate().step_by((query.ilog10() + 1) as usize) {
                    let Ok(value) = proc.value_at(region.0.BaseAddress as usize + offset)
                    else { continue; };
                    if value != query as u32 { continue; }
                    let addr = format!("{:#X}", region.0.BaseAddress as usize + offset);
                    app.modify_progress_msg(format!("Found value at address {:#X}.", region.0.BaseAddress as usize + offset)).await;
                    app.app.lock().await.query_results.push((addr, value.to_string()));
                }
            }
        }
        _ => {app.modify_progress_msg(format!("Could not parse type.")).await;}
    }

    app.modify_query_progress(0.00).await;
    app.modify_progress_msg(format!("Query complete.")).await;
    app.modify_querying(false).await;
    
}