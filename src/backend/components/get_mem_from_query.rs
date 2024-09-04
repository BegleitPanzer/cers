use winapi::um::winnt;

use crate::{backend::process::process::Process, ui::main::{App, Data}};

pub fn get_mem_from_query(
    upper_bound: usize,
    lower_bound: usize,
    pid: &i32, 
    app_query: &String,
    tx : &std::sync::mpsc::Sender<Data>,
) {

    let Ok(proc) = Process::open(pid.clone() as u32)
    else { return; };
    tx.send(Data {data_type: crate::ui::main::DataType::ProgressMsg, data: format!("lower_bound: {:#X}, upper_bound: {:#X}", lower_bound, upper_bound) } );
    // filter memory regions by upper and lower bounds
    let mem = proc
        .memory_regions().
        into_iter().
        filter(|p| {
            ((p.Protect & winnt::PAGE_READWRITE) != 0)
            && ((p.BaseAddress as usize) >= lower_bound && (p.BaseAddress as usize) <= upper_bound)
        });

    let regions = mem.map(|p| p).collect::<Vec<_>>();
    for region in regions.clone().into_iter() {
        //app.query_progress = regions.clone().into_iter().position(|p| p.BaseAddress == region.BaseAddress).unwrap() as f64 / regions.len() as f64;
        tx.send(Data {data_type: crate::ui::main::DataType::QueryProgress, data: (regions.clone().into_iter().position(|p| p.BaseAddress == region.BaseAddress).unwrap() as f64 / regions.len() as f64).to_string()});
        let Ok(memory) = proc.read_memory(region.BaseAddress as _, region.RegionSize)
        else { continue; };
        for (offset, window) in memory.windows(4).enumerate() {
            let value = proc.value_at(region.BaseAddress as usize + offset).unwrap();
            let value = String::from_utf16(value.to_le_bytes().iter().map(|b| *b as u16).collect::<Vec<u16>>().as_ref()).unwrap();
            if value != *app_query { continue; }
            let addr = format!("{:#X}", region.BaseAddress as usize + offset);

            tx.send(Data {data_type: crate::ui::main::DataType::QueryResults, data: addr + "||" + &value  });
        }
        
    }


}