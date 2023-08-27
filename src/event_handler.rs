use crossterm::event::KeyCode;

use crate::state_handler::{AppIndexMove, AppMode, AppState};

pub fn handle_key_code(code: KeyCode, state: &mut AppState) {
    match state.mode {
        AppMode::Navigation => handle_key_code_nav(code, state),
        AppMode::Deletion => handle_key_code_deletion(code, state),
        AppMode::Search => handle_key_code_search(code, state),
    }
}

fn handle_key_code_search(code: KeyCode, state: &mut AppState) {
    match code {
        KeyCode::Char(ch) => state.handle_search_term_update(ch),

        KeyCode::Backspace => state.handle_backspace(),
        KeyCode::Esc => {
            state.search_term = "".to_owned();
            state.mode = AppMode::Navigation
        }
        KeyCode::Enter => state.mode = AppMode::Navigation,
        _ => state.handle_unsupported(),
    }
}

fn handle_key_code_deletion(code: KeyCode, state: &mut AppState) {
    match code {
        KeyCode::Char('d') => state.handle_kill_process(),
        KeyCode::Esc => state.mode = AppMode::Navigation,
        _ => state.handle_unsupported(),
    }
}

fn handle_key_code_nav(code: KeyCode, state: &mut AppState) {
    match code {
        KeyCode::Char('j') => state.move_list_index(AppIndexMove::Next),
        KeyCode::Char('k') => state.move_list_index(AppIndexMove::Previous),
        KeyCode::Char('l') => state.move_page_index(AppIndexMove::Next),
        KeyCode::Char('h') => state.move_page_index(AppIndexMove::Previous),
        KeyCode::Char('d') => state.enter_deletion_mode(),
        KeyCode::Char('/') => state.mode = AppMode::Search,
        KeyCode::Esc => state.escape_navigation_mode(),
        _ => state.handle_unsupported(),
    }
}
