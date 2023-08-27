use std::borrow::Cow;
use std::error::Error;
use std::panic;
use std::time::{Duration, Instant};

use crossterm::event::{self, KeyEvent};
use psutil::process::processes;

mod event_handler;
mod processes;
mod state_handler;
mod system;

fn start_interactive(state: &mut state_handler::AppState) -> Result<(), Box<dyn Error>> {
    state.display()?;
    let mut last_refreshed = Instant::now();
    loop {
        if event::poll(Duration::from_millis(40))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                event_handler::handle_key_code(code, state);
                state.display()?;
            }
        } else {
            // refreshes every 600 milliseconds irrespective of user input
            // TODO: selectively refresh processes if search term is not empty
            if Instant::now() - last_refreshed >= Duration::from_millis(500)
                && state.search_term == String::from("")
            {
                let processes = processes::generate_process_data()?;
                let updated_pages =
                    processes::generate_paginated_process_list(Cow::Borrowed(&processes));
                state.displayed_processes = updated_pages;
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
