extern crate ncurses;
use ncurses::*;

#[derive(Debug, PartialEq)]
enum EntryState {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
struct Entry {
    description: String,
    state: EntryState,
}

impl Entry {
    fn new(description: String) -> Self {
        Entry {
            description,
            state: EntryState::Todo,
        }
    }

    fn change_state(&mut self, new_state: EntryState) {
        if self.state != new_state {
            self.state = new_state;
        }
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

enum Window {
    CurrentTasks,
    ArchivedTasks,
    Help,
}

struct Todo {
    pub current_tasks: Vec<Entry>,
    archived_tasks: Vec<Entry>,
    pub cursor: usize,
    active_window: Window,
}

impl Todo {
    fn add_task(&mut self, new_entry: Entry) {
        self.current_tasks.push(new_entry);
    }

    fn cursor_up(&mut self) {
        if (self.cursor > 0) {
            self.cursor -= 1;
        }
    }

    fn cursor_down(&mut self) {
        match self.active_window {
            Window::CurrentTasks => {
                if (self.cursor < self.current_tasks.len() - 1) {
                    self.cursor += 1;
                }
            }
            Window::ArchivedTasks => {
                if (self.cursor < self.archived_tasks.len() - 1) {
                    self.cursor += 1;
                }
            }
            Window::Help => {}
        }
    }
}

const HELP_WIDTH: i32 = 40;
const HELP_HEIGHT: i32 = 5;

fn refresh_current(current: WINDOW, todo: &Todo) {
    for (i, item) in todo.current_tasks.iter().enumerate() {
        if (i == todo.cursor) {
            wattron(current, A_BOLD | A_UNDERLINE);
        } else {
            wattroff(current, A_BOLD | A_UNDERLINE);
        }
        let _ = mvwprintw(current, i as i32 + 1, 1, item.get_description());
    }
    wrefresh(current);
}

fn refresh_archived(archived: WINDOW, todo: &Todo) {
    // TODO
    wrefresh(archived);
}

fn main() {
    let root = initscr();
    keypad(initscr(), true);
    refresh();

    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    let _ = getmaxyx(root, &mut max_y, &mut max_x);

    let help = newwin(
        HELP_HEIGHT,
        HELP_WIDTH,
        max_y - HELP_HEIGHT,
        (max_x / 2) - (HELP_WIDTH / 2),
    );
    box_(help, 0, 0);
    let _ = mvwprintw(help, 0, 0, "HELP");
    let _ = mvwprintw(help, 1, 1, "up/down - navigate");
    let _ = mvwprintw(help, 2, 1, "q - quit");
    let _ = mvwaddstr(help, 3, 1, "ASD");
    wrefresh(help);

    let current_tasks = newwin(max_y - HELP_HEIGHT, max_x / 2, 0, 0);
    box_(current_tasks, 0, 0);
    let _ = mvwprintw(current_tasks, 0, 0, "CURRENT");

    let archived_tasks = newwin(max_y - HELP_HEIGHT, max_x / 2, 0, max_x / 2);
    box_(archived_tasks, 0, 0);
    let _ = mvwprintw(archived_tasks, 0, 0, "ARCHIVED");

    // tmp data
    let mut todo = Todo {
        current_tasks: vec![],
        archived_tasks: vec![],
        cursor: 0,
        active_window: Window::CurrentTasks,
    };
    todo.add_task(Entry::new("first task".to_string()));
    todo.add_task(Entry::new("second task".to_string()));
    todo.add_task(Entry::new("third task".to_string()));

    loop {
        // TODO: break condition
        refresh_current(current_tasks, &todo);
        refresh_archived(archived_tasks, &todo);
        let ch = getch();
        match ch {
            KEY_UP => todo.cursor_up(),
            KEY_DOWN => todo.cursor_down(),
            // 113 is for `q`
            113 => {
                break;
            }
            _ => {}
        }
    }

    endwin();
}
