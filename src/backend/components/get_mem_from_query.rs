
use winapi::um::winnt;

use crate::{backend::process::process::Process, ui::main::{AMApp, App, Data}};

#[derive(Clone)]
struct WrappedMBI(winnt::MEMORY_BASIC_INFORMATION);
unsafe impl Send for WrappedMBI {}

pub async fn get_mem_from_query( upper_bound: usize, lower_bound: usize, app: AMApp ) {
    panic!();
    let Ok(proc) = Process::open(app.get_process().await as u32)
    else { return; };
    let proc = proc;
    app.modify_progress_msg(format!("lower_bound: {:#X}, upper_bound: {:#X}", lower_bound, upper_bound)).await;
    // filter memory regions by upper and lower bounds
    let mem = proc
        .memory_regions().
        into_iter().
        filter(|p| {
            ((p.Protect & winnt::PAGE_READWRITE) != 0)
            && ((p.BaseAddress as usize) >= lower_bound && (p.BaseAddress as usize) <= upper_bound)
        });
    
    let regions: Vec<WrappedMBI> = mem.map(|p| WrappedMBI(p)).collect::<Vec<_>>();

    let query = app.get_query().await; // don't want to lock this a million times
    
    for region in regions.clone().into_iter() {
        app.modify_query_progress(regions.clone().into_iter().position(|p| p.0.BaseAddress == region.0.BaseAddress).unwrap() as f64 / regions.len() as f64).await;
        let Ok(memory) = proc.read_memory(region.0.BaseAddress as _, region.0.RegionSize)
        else { continue; };
        for (offset, window) in memory.windows(4).enumerate() {
            let value = proc.value_at(region.0.BaseAddress as usize + offset).unwrap();
            let value = String::from_utf16(value.to_le_bytes().iter().map(|b| *b as u16).collect::<Vec<u16>>().as_ref()).unwrap();
            if value != query.1 { continue; }
            let addr = format!("{:#X}", region.0.BaseAddress as usize + offset);

            app.app.lock().await.query_results.push((addr, value));
        }
    }
}