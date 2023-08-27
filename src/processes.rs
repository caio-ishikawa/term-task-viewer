use std::borrow::Cow;
use std::error::Error;

use psutil::process::processes;

#[derive(Clone, Debug)]
pub struct ProcessData {
    pub pid: u32,
    pub name: String,
    pub memory_percent: f32,
    pub memory_mbs: f64,
    pub cpu: f32,
}

pub fn generate_process_data() -> Result<Vec<ProcessData>, Box<dyn Error>> {
    let processes = processes()?;
    let mut process_list: Vec<ProcessData> = Vec::new();

    for process in processes {
        if let Ok(mut process) = process {
            let memory_percent = process.memory_percent()?;
            let cpu = process.cpu_percent()?;
            let name = process.name()?;
            let mem_info = process.memory_info()?;
            let memory_mbs = mem_info.rss() as f64 / (1024.0 * 1024.0);

            process_list.push(ProcessData {
                pid: process.pid(),
                name,
                memory_percent,
                memory_mbs,
                cpu,
            });
        }
    }

    Ok(process_list)
}

pub fn generate_paginated_process_list(processes: Cow<Vec<ProcessData>>) -> Vec<Vec<ProcessData>> {
    let (_width, height) = crossterm::terminal::size().expect("Error getting terminal size");

    let mut paginated_processes: Vec<Vec<ProcessData>> = Vec::new();
    let mut curr_page: Vec<ProcessData> = Vec::new();

    let term_height = (height - 10) as usize;

    for proc in processes.iter() {
        if curr_page.len() == term_height {
            paginated_processes.push(curr_page);
            curr_page = Vec::new();
        }
        curr_page.push(proc.to_owned());
    }

    if !curr_page.is_empty() {
        paginated_processes.push(curr_page);
    }

    paginated_processes
}

pub fn filter_processes(processes: Cow<Vec<ProcessData>>, search_term: &str) -> Vec<ProcessData> {
    let mut filtered_processes: Vec<ProcessData> = processes
        .into_owned()
        .into_iter()
        .filter(|proc| {
            proc.name
                .to_lowercase()
                .contains(&search_term.to_lowercase())
        })
        .collect();

    filtered_processes.sort_by(|a, b| {
        let cleaned_a = a.name.to_lowercase();
        let cleaned_b = b.name.to_lowercase();
        let cleaned_search_term = search_term.to_lowercase();

        let a_pos = cleaned_a.find(&cleaned_search_term);
        let b_pos = cleaned_b.find(&cleaned_search_term);

        match (a_pos, b_pos) {
            (Some(a_idx), Some(b_idx)) => a_idx.cmp(&b_idx),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            _ => cleaned_a.cmp(&cleaned_b),
        }
    });

    filtered_processes
}
