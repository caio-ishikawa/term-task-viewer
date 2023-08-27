use std::borrow::Cow;
use std::error::Error;
use std::io::stdout;
use std::io::Write;

use crossterm::style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor};
use crossterm::{cursor, QueueableCommand};
use psutil::process::Process;

use crate::processes;
use crate::system::{self, SystemData};

//TODO: Add message_bar field to AppState.
//TODO: Catch errors and display them on message_bar field.
//TODO: Print message bar on the bottom of the terminal window
//

pub enum AppIndexMove {
    Next,
    Previous,
}

#[derive(Clone, Copy)]
pub enum AppMode {
    Navigation,
    Deletion,
    Search,
    //Monitoring
}

#[derive(Clone)]
pub struct AppState {
    pub mode: AppMode,
    pub system_data: SystemData,
    pub processes: Vec<processes::ProcessData>,
    pub displayed_processes: Vec<Vec<processes::ProcessData>>,
    pub search_term: String,
    pub selected_index: usize,
    pub curr_page_index: usize,
    pub message: String,
}

impl AppState {
    pub fn new() -> Result<AppState, Box<dyn Error>> {
        let processes = processes::generate_process_data()?;
        let paginated_processes =
            processes::generate_paginated_process_list(Cow::Borrowed(&processes));

        let system_data = system::generate_system_data()?;

        return Ok(AppState {
            mode: AppMode::Navigation,
            system_data,
            processes: processes.clone(),
            displayed_processes: paginated_processes,
            search_term: "".to_owned(),
            selected_index: 0,
            curr_page_index: 0,
            message: "".to_owned(),
        });
    }

    pub fn display(&self) -> Result<(), Box<dyn Error>> {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let mut stdout = stdout();
        let mut starting_row = 5;

        print!(
            "{}{}{}{}{}",
            SetAttribute(Attribute::Underlined),
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Blue),
            "System Data                                          ",
            ResetColor
        );
        stdout.queue(cursor::MoveTo(0, 1))?;
        print!(
            "{}{}{}:{} {:.2}%",
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Blue),
            "CPU %",
            ResetColor,
            self.system_data.cpu_percent
        );
        stdout.queue(cursor::MoveTo(16, 1))?;
        print!(
            "{}{}{}:{} {}Mb",
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Blue),
            "MEM Used",
            ResetColor,
            self.system_data.ram_used / 1024 / 1024
        );
        stdout.queue(cursor::MoveTo(35, 1))?;
        print!(
            "{}{}{}:{} {}Mb",
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Blue),
            "MEM Total",
            ResetColor,
            self.system_data.ram_total / 1024 / 1024
        );

        stdout.queue(cursor::MoveTo(0, 3))?;
        print!(
            "{}{}{}{}{}",
            SetAttribute(Attribute::Underlined),
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Blue),
            "Tasks                                                ",
            ResetColor
        );
        stdout.queue(cursor::MoveTo(0, 4))?;

        print!(
            "{}{}{}",
            SetForegroundColor(Color::Blue),
            SetAttribute(Attribute::Bold),
            "PID"
        );
        stdout.queue(cursor::MoveToColumn(7))?;
        print!("NAME");
        stdout.queue(cursor::MoveToColumn(30))?;
        print!("MEM %");
        stdout.queue(cursor::MoveToColumn(37))?;
        print!("MEM MBs");
        stdout.queue(cursor::MoveToColumn(48))?;
        print!("CPU %{}", ResetColor);
        stdout.queue(cursor::MoveToColumn(48))?;

        for (i, proc) in self.displayed_processes[self.curr_page_index]
            .iter()
            .enumerate()
        {
            stdout.queue(cursor::MoveToRow(starting_row))?;
            if i == self.selected_index {
                stdout.queue(cursor::MoveToColumn(0))?;
                print!(
                    "{}{}{}",
                    SetForegroundColor(Color::Red),
                    SetAttribute(Attribute::Bold),
                    proc.pid,
                );

                stdout.queue(cursor::MoveToColumn(7))?;
                print!("{}", proc.name);

                stdout.queue(cursor::MoveToColumn(30))?;
                print!("{:.2}%", proc.memory_percent);

                stdout.queue(cursor::MoveToColumn(37))?;
                print!("{:.2}", proc.memory_mbs);

                stdout.queue(cursor::MoveToColumn(42))?;
                print!(" MBs");

                stdout.queue(cursor::MoveToColumn(48))?;
                println!("{:.2}%{}", proc.cpu, ResetColor);

                stdout.queue(cursor::MoveToColumn(10))?;
            } else {
                stdout.queue(cursor::MoveToColumn(0))?;
                print!("{}", proc.pid);

                stdout.queue(cursor::MoveToColumn(7))?;
                print!("{}", proc.name);

                stdout.queue(cursor::MoveToColumn(30))?;
                print!("{:.2}%", proc.memory_percent);

                stdout.queue(cursor::MoveToColumn(37))?;
                print!("{:.2}", proc.memory_mbs);

                stdout.queue(cursor::MoveToColumn(42))?;
                print!(" MBs");

                stdout.queue(cursor::MoveToColumn(48))?;
                print!("{:.2}%", proc.cpu);
                std::io::stdout().flush().unwrap();

                stdout.queue(cursor::MoveToColumn(10))?;
            }
            starting_row += 1;
        }

        let total_pages = self.displayed_processes.len() - 1;
        let y_coord = self.displayed_processes[0].len() as u16 + 5;

        stdout.queue(cursor::MoveTo(0, y_coord))?;
        print!(
            "{}{}{}{}",
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Blue),
            "PAGE:",
            ResetColor
        );

        stdout.queue(cursor::MoveTo(49, y_coord))?;
        print!(
            "{}{}{}{}/{}{}{}{}",
            SetForegroundColor(Color::Red),
            SetAttribute(Attribute::Bold),
            self.curr_page_index,
            ResetColor,
            SetForegroundColor(Color::Blue),
            SetAttribute(Attribute::Bold),
            total_pages,
            ResetColor
        );
        std::io::stdout().flush().unwrap();

        let (_width, height) = crossterm::terminal::size().expect("Error getting terminal size");
        stdout.queue(cursor::MoveTo(0, height))?;
        if self.message != "".to_owned() {
            print!("{}{}", SetAttribute(Attribute::Bold), self.message);
        } else {
            match self.mode {
                AppMode::Search => {
                    print!("{}:/{}", SetAttribute(Attribute::Bold), self.search_term);
                    stdout.queue(cursor::MoveToRow(height))?;
                }
                _ => ()
            }
        }
        std::io::stdout().flush().unwrap();
        Ok(())
    }

    pub fn handle_search_term_update(&mut self, ch: char) {
        self.search_term.push(ch);

        let filtered_processes =
            processes::filter_processes(Cow::Borrowed(&self.processes), &self.search_term);
        let updated_paginated =
            processes::generate_paginated_process_list(Cow::Borrowed(&filtered_processes));
        self.processes = filtered_processes;
        self.displayed_processes = updated_paginated;
    }

    pub fn handle_backspace(&mut self) {
        self.search_term.pop();

        let filtered_processes =
            processes::filter_processes(Cow::Borrowed(&self.processes), &self.search_term);
        let updated_paginated =
            processes::generate_paginated_process_list(Cow::Borrowed(&filtered_processes));
        self.processes = filtered_processes;
        self.displayed_processes = updated_paginated;
    }

    pub fn handle_kill_process(&mut self) {
        let pid = self.displayed_processes[self.curr_page_index][self.selected_index].pid;
        let proc = Process::new(pid).expect("error getting with pid");
        proc.terminate().expect("error terminating process");

        let processes = processes::generate_process_data().expect("error getting process data");
        let updated_pages = processes::generate_paginated_process_list(Cow::Borrowed(&processes));
        self.displayed_processes = updated_pages;
        self.mode = AppMode::Navigation;
        self.message = "Killed process.".to_owned();
    }

    pub fn enter_deletion_mode(&mut self) {
        self.mode = AppMode::Deletion;
        self.message = format!(
            "Killing process name: {}. Please confirm.",
            self.displayed_processes[self.curr_page_index][self.selected_index].name
        );
    }

    pub fn handle_unsupported(&mut self) {
        self.message = "Input not supported.".to_owned();
        self.mode = AppMode::Navigation; // sets mode back to default
    }

    pub fn move_page_index(&mut self, index_move: AppIndexMove) {
        match index_move {
            AppIndexMove::Next => {
                if self.curr_page_index == self.displayed_processes.len() - 1 {
                    self.curr_page_index = 0;
                } else {
                    self.curr_page_index += 1;
                }
            },
            AppIndexMove::Previous => {
                if self.curr_page_index == 0 {
                    self.curr_page_index = self.displayed_processes.len() - 1;
                } else {
                    self.curr_page_index -= 1;
                }
            }
        }
    }

    pub fn move_list_index(&mut self, index_move: AppIndexMove) {
        match index_move {
            AppIndexMove::Next => {
                if self.selected_index == self.displayed_processes[self.curr_page_index].len() {
                    self.selected_index = 0;
                } else {
                    self.selected_index += 1;
                }
            },
            AppIndexMove::Previous => {
                if self.selected_index == 0 {
                    self.selected_index = self.displayed_processes[self.curr_page_index].len();
                } else {
                    self.selected_index -= 1;
                }
            }
        }
    }
}
