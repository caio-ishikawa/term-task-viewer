use crate::processes;
use crossterm::style::{
    Attribute, Color, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor, Stylize,
};
use crossterm::{cursor, QueueableCommand};
use std::error::Error;
use std::io::stdout;

//TODO: Add message_bar field to AppState.
//TODO: Catch errors and display them on message_bar field.
//TODO: Print message bar on the bottom of the terminal window

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

        print!(
            "{}{}{}",
            SetForegroundColor(Color::Blue),
            SetAttribute(Attribute::Bold),
            "PID"
        );
        stdout.queue(cursor::MoveToColumn(7))?;
        print!("Name");
        stdout.queue(cursor::MoveToColumn(30))?;
        print!("Mem %");
        stdout.queue(cursor::MoveToColumn(37))?;
        print!("Mem MBs");
        stdout.queue(cursor::MoveToColumn(48))?;
        print!("CPU %{}", ResetColor);
        stdout.queue(cursor::MoveToColumn(48))?;

        for (i, proc) in self.processes[self.curr_page_index].iter().enumerate() {
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
                println!("{:.2}%", proc.cpu);

                stdout.queue(cursor::MoveToColumn(10))?;
            }
            starting_row += 1;
        }

        let total_pages = self.processes.len();
        let y_coord = self.processes[0].len() as u16 + 1;
        stdout.queue(cursor::MoveTo(0, y_coord))?;

        let mut x_coord = 0;
        for page in 0..total_pages {
            stdout.queue(cursor::MoveTo(x_coord, y_coord))?;

            let mut page_num_str = format!("|{}|", page.to_string());
            if page < 10 {
                // purely aesthetic
                page_num_str = format!("|0{}|", page);
            }

            if page == self.curr_page_index {
                println!(
                    "{}{}{}{}",
                    SetAttribute(Attribute::Bold),
                    SetForegroundColor(Color::Red),
                    page_num_str,
                    ResetColor
                );
            } else {
                println!("{}", page_num_str);
            }

            x_coord += 3;
        }

        stdout.queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }
}
