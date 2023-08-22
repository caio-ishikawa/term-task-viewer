use psutil::process::processes;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct ProcessData {
    pub pid: u32,
    pub name: String,
    pub memory_percent: f32,
    pub memory_mbs: f64,
    pub cpu: f32,
}

pub fn generate_process_data() -> Result<Vec<ProcessData>, Box<dyn Error>> {
    let processes = processes().expect("error");
    let mut process_list: Vec<ProcessData> = Vec::new();

    for process in processes {
        if let Ok(mut process) = process {
            let memory_percent = process.memory_percent().expect("Error getting memory data");
            let cpu = process.cpu_percent().expect("Error gettting CPU data");
            let name = process.name().expect("Error getting name data");
            let mem_info = process.memory_info().expect("Error getting memory info");
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

pub fn generate_paginated_process_list() -> Vec<Vec<ProcessData>> {
    let processes = generate_process_data().expect("Error generating processes");
    let (_width, height) = crossterm::terminal::size().expect("Error getting terminal size");

    let mut paginated_processes: Vec<Vec<ProcessData>> = Vec::new();
    let mut curr_page: Vec<ProcessData> = Vec::new();

    let term_height = (height - 10) as usize;

    for proc in processes {
        if curr_page.len() == term_height {
            paginated_processes.push(curr_page);
            curr_page = Vec::new();
        }
        curr_page.push(proc);
    }

    if !curr_page.is_empty() {
        paginated_processes.push(curr_page);
    }

    paginated_processes
}
