use std::collections::HashSet;

use crate::backend::process::process::enum_proc;

use super::process::process::{Process};

pub fn get_process_list() -> Vec<(String, u32)> {
    let mut fproclist: Vec<(String, u32)> = vec![];
    let proclist = enum_proc()
        .unwrap()
        .into_iter()
        .filter_map(|pid| Process::open(pid).ok())
        .map(|p| (p.name().unwrap(), p.pid))
        .collect::<Vec<(String, u32)>>();
    for proc in proclist {
        if fproclist.clone().into_iter().any(|p| p.0 == proc.0) { continue; }
        else { fproclist.push(proc) }
    }
    fproclist
}