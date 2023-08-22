use crate::processes;
use crate::state_handler;
use crossterm::event::KeyCode;
use psutil::process::Process;
use std::panic;

pub fn handle_key_code(code: KeyCode, state: &mut state_handler::AppState) {
    match code {
        KeyCode::Char('j') => {
            state.selected_index += 1;
        }
        KeyCode::Char('k') => {
            state.selected_index -= 1;
        }
        KeyCode::Char('l') => {
            state.curr_page_index += 1;
        }
        KeyCode::Char('h') => {
            state.curr_page_index -= 1;
        }
        KeyCode::Char('d') => {
            // needs to change mode to Delete mode, and expect another D press to
            // delete
            let pid = state.processes[state.curr_page_index][state.selected_index].pid;
            let proc = Process::new(pid).expect("error getting with pid");
            proc.terminate().expect("error terminating process");

            let updated_pages = processes::generate_paginated_process_list();
            state.processes = updated_pages;
        }
        KeyCode::Char('/') => {
            panic!("SEARCH MODE")
        }
        _ => {
            panic!("error");
        }
    }
}
