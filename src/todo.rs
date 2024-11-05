use crate::entry::*;
use crate::Window;

pub struct Todo {
    pub current_tasks: Vec<Entry>,
    pub archived_tasks: Vec<Entry>,
    pub cursor: usize,
    pub active_window: Window,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {
            current_tasks: vec![],
            archived_tasks: vec![],
            cursor: 0,
            active_window: Window::CurrentTasks,
        }
    }

    pub fn new_test() -> Todo {
        let mut tmp1 = Entry::new("fourth task".to_string());
        tmp1.change_state(EntryState::Done);
        let mut tmp2 = Entry::new("fifth task".to_string());
        tmp2.change_state(EntryState::Done);

        Todo {
            current_tasks: vec![
                Entry::new("first task".to_string()),
                Entry::new("second task".to_string()),
                Entry::new("third task".to_string()),
            ],
            archived_tasks: vec![tmp1, tmp2],
            cursor: 0,
            active_window: Window::CurrentTasks,
        }
    }

    pub fn add_task(&mut self, new_entry: Entry) {
        self.current_tasks.push(new_entry);
    }

    pub fn cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn cursor_down(&mut self) {
        match self.active_window {
            Window::CurrentTasks => {
                if self.cursor < self.current_tasks.len() - 1 {
                    self.cursor += 1;
                }
            }
            Window::ArchivedTasks => {
                if self.cursor < self.archived_tasks.len() - 1 {
                    self.cursor += 1;
                }
            }
        }
    }

    pub fn mark_selected_as_done(&mut self) {
        if self.active_window != Window::CurrentTasks || self.current_tasks.is_empty() {
            return;
        }

        let current_task = self.current_tasks.remove(self.cursor);
        self.archived_tasks.push(current_task);

        if self.current_tasks.is_empty() {
            // TODO: write text saying "EMPTY" or sth similar
            return;
        }

        if self.cursor > self.current_tasks.len() - 1 {
            self.cursor = self.current_tasks.len() - 1;
        }
    }

    pub fn mark_selected_as_todo(&mut self) {
        if self.active_window != Window::ArchivedTasks || self.archived_tasks.is_empty() {
            return;
        }

        let archived_task = self.archived_tasks.remove(self.cursor);
        self.current_tasks.push(archived_task);

        if self.archived_tasks.is_empty() {
            // TODO: write text saying "EMPTY" or sth similar
            return;
        }

        if self.cursor > self.archived_tasks.len() -1 {
            self.cursor = self.archived_tasks.len() - 1;
        }
    }

    pub fn cursor_change_window(&mut self) {
        match self.active_window {
            Window::CurrentTasks => self.active_window = Window::ArchivedTasks,
            Window::ArchivedTasks => self.active_window = Window::CurrentTasks,
        }
        self.cursor = 0;
    }
}
