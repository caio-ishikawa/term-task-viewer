use crossterm::event::{self, KeyCode, KeyEvent};
use psutil::process::processes;
use psutil::process::Process;
use std::error::Error;
use std::panic;
use std::time::{Duration, Instant};
mod processes;
mod state_handler;

fn start_interactive(state: &mut state_handler::AppState) -> Result<(), Box<dyn Error>> {
    state.display()?;
    let mut last_refreshed = Instant::now();
    loop {
        if event::poll(Duration::from_millis(20))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('j') => {
                        state.selected_index += 1;
                        state.display().unwrap();
                    }
                    KeyCode::Char('k') => {
                        state.selected_index -= 1;
                        state.display().unwrap();
                    }
                    KeyCode::Char('l') => {
                        state.curr_page_index += 1;
                        state.display().unwrap();
                    }
                    KeyCode::Char('h') => {
                        state.curr_page_index -= 1;
                        state.display().unwrap();
                    }
                    KeyCode::Char('d') => {
                        // needs to change mode to Delete mode, and expect another D press to
                        // delete
                        let pid = state.processes[state.curr_page_index][state.selected_index].pid;
                        let proc = Process::new(pid).expect("error getting with pid");
                        proc.terminate().expect("error terminating process");

                        let updated_pages = processes::generate_paginated_process_list();
                        state.processes = updated_pages;
                        state.display()?;
                    }
                    _ => panic!("error"),
                }
                last_refreshed = Instant::now();
            }
        } else {
            if Instant::now() - last_refreshed >= Duration::from_millis(600) {
                let updated_pages = processes::generate_paginated_process_list();
                state.processes = updated_pages;
                state.display()?;
                last_refreshed = Instant::now();
            }
        }
    }
}

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode.");
        println!("Panic occurred: {:?}", panic_info);
    }));

    crossterm::terminal::enable_raw_mode().expect("Could not enable raw mode");

    let mut initial_state = state_handler::AppState::new().expect("Error creating app state");

    start_interactive(&mut initial_state).unwrap();

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode.");
}
