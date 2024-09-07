
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
    String
}

///
/// Scan's the selected processes memory for a specific value
/// This needs to be HEAVILY optimized. Cheat Engine is lightning fast, this isn't.
/// 
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
    match query_type {
        QueryTypes::Bytes2 | QueryTypes::Bytes4 | QueryTypes::Bytes8  => {
            for region in regions.clone().into_iter() {
                let query = query.parse::<u32>().unwrap();
                app.modify_query_progress(regions.clone().into_iter().position(|p| p.0.BaseAddress == region.0.BaseAddress).unwrap() as f64 / regions.len() as f64).await;
                let Ok(memory) = proc.read_memory(region.0.BaseAddress as _, region.0.RegionSize)
                else { continue; };
                for (offset, _)  in memory.windows((query.ilog10() + 1) as usize).enumerate().into_iter().step_by((query.ilog10() + 1) as usize) {
                    let Ok(value) = proc.value_at(region.0.BaseAddress as usize + offset as usize)
                    else { continue; };
                    if value != query as u32 { continue; }
                    let addr = format!("{:#X}", region.0.BaseAddress as usize + offset as usize);
                    app.modify_progress_msg(format!("Found value at address {:#X}.", region.0.BaseAddress as usize + offset as usize)).await;
                    app.app.lock().await.query_results.push((addr, value.to_string()));
                }
            }
        }
        // this is pretty slow, i think.
        QueryTypes::String => {
            for region in regions.clone().into_iter() {
                app.modify_query_progress(regions.clone().into_iter().position(|p| p.0.BaseAddress == region.0.BaseAddress).unwrap() as f64 / regions.len() as f64).await;
                let Ok(memory) = proc.read_memory(region.0.BaseAddress as _, region.0.RegionSize) 
                else { continue; };
                for (offset, window) in memory.windows(query.len()).enumerate() {
                    let Ok(value) = String::from_utf8(window.to_vec())
                    else { continue; };
                    if value != query { continue; };
                    let addr = format!("{:#X}", region.0.BaseAddress as usize + offset);
                    app.modify_progress_msg(format!("Found value at address {:#X}.", region.0.BaseAddress as usize + offset)).await;
                    app.app.lock().await.query_results.push((addr, value.to_string()));
                }
            }
        }
        // this can be WAYYYY faster
        QueryTypes::Float  => {
            for region in regions.clone().into_iter() {
                let query = query.parse::<f32>().unwrap().to_le_bytes(); // technically doesn't always have to be a f64
                app.modify_query_progress(regions.clone().into_iter().position(|p| p.0.BaseAddress == region.0.BaseAddress).unwrap() as f64 / regions.len() as f64).await;
                let Ok(memory) = proc.read_memory(region.0.BaseAddress as _, region.0.RegionSize)
                else { continue };
                for (offset, _)  in memory.windows(query.len()).enumerate().into_iter().step_by((query.len()) as usize) {
                    let Ok(value) = proc.value_at(region.0.BaseAddress as usize + offset as usize)
                    else { continue };
                    let value = value.to_le_bytes();
                    if value != query { continue };
                    let addr = format!("{:#X}", region.0.BaseAddress as usize + offset as usize);
                    app.modify_progress_msg(format!("Found value at address {:#X}.", region.0.BaseAddress as usize + offset as usize)).await;
                    app.app.lock().await.query_results.push((addr, f32::from_le_bytes(value).to_string()));
                

                }
                
            }

        }
        _ => {app.modify_progress_msg(format!("Could not parse type.")).await;}
    }

    app.modify_query_progress(0.00).await;
    app.modify_progress_msg(format!("Query complete.")).await;
    app.modify_querying(false).await;
    
}