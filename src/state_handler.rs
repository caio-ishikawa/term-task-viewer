use crate::processes;
use crossterm::style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor};
use crossterm::{cursor, QueueableCommand};
use std::error::Error;
use std::io::stdout;

#[derive(Clone, Copy)]
pub enum AppMode {
    Navigation,
    //Deletion,
    //Search,
    //Monitoring
}

#[derive(Clone)]
pub struct AppState {
    pub mode: AppMode,
    pub processes: Vec<Vec<processes::ProcessData>>,
    pub search_term: String,
    pub selected_index: usize,
    pub curr_page_index: usize,
}

impl AppState {
    pub fn new() -> Result<AppState, Box<dyn Error>> {
        let processes = processes::generate_process_data()?;
        let (_width, height) = crossterm::terminal::size().expect("Error getting terminal size");

        let mut paginated_processes: Vec<Vec<processes::ProcessData>> = Vec::new();
        let mut curr_page = Vec::new();

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

        return Ok(AppState {
            mode: AppMode::Navigation,
            processes: paginated_processes,
            search_term: "".to_owned(),
            selected_index: 0,
            curr_page_index: 0,
        });
    }

    pub fn display(&self) -> Result<(), Box<dyn Error>> {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let mut stdout = stdout();
        let mut starting_row = 1;

        for (i, proc) in self.processes[self.curr_page_index].iter().enumerate() {
            stdout.queue(cursor::MoveToRow(starting_row))?;
            if i == self.selected_index {
                stdout.queue(cursor::MoveToColumn(0))?;
                print!("{}{}", proc.pid, SetAttribute(Attribute::Bold));
                stdout.queue(cursor::MoveToColumn(7))?;
                print!(
                    "{}{}{}",
                    SetForegroundColor(Color::Red),
                    proc.name,
                    ResetColor
                );
                stdout.queue(cursor::MoveToColumn(30))?;
                print!("{:.2}%", proc.memory_percent);
                stdout.queue(cursor::MoveToColumn(37))?;
                print!("{:.2}", proc.memory_mbs);
                stdout.queue(cursor::MoveToColumn(42))?;
                print!(" MBs");
                stdout.queue(cursor::MoveToColumn(48))?;
                println!("{:.2}%", proc.cpu);
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
                println!("{:.2}%", proc.cpu);
                stdout.queue(cursor::MoveToColumn(10))?;
            }
            starting_row += 1;
        }
        stdout.queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }
}
