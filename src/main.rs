extern crate ncurses;
use ncurses::*;

#[derive(Debug, PartialEq)]
enum EntryState {
    Todo,
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

#[derive(PartialEq)]
enum Window {
    CurrentTasks,
    ArchivedTasks,
}

struct Todo {
    current_tasks: Vec<Entry>,
    archived_tasks: Vec<Entry>,
    cursor: usize,
    active_window: Window,
}

impl Todo {
    fn new() -> Todo {
	Todo {
	    current_tasks: vec![],
	    archived_tasks: vec![],
	    cursor: 0,
	    active_window: Window::CurrentTasks,
	}
    }

    fn new_test() -> Todo {
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

    fn add_task(&mut self, new_entry: Entry) {
	self.current_tasks.push(new_entry);
    }

    fn cursor_up(&mut self) {
	if self.cursor > 0 {
	    self.cursor -= 1;
	}
    }

    fn cursor_down(&mut self) {
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

    fn mark_selected_as_done(&mut self) {
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

    fn cursor_change_window(&mut self) {
	match self.active_window {
	    Window::CurrentTasks => self.active_window = Window::ArchivedTasks,
	    Window::ArchivedTasks => self.active_window = Window::CurrentTasks,
	}
	self.cursor = 0;
    }
}

fn refresh_current(current: WINDOW, todo: &Todo) {
    wclear(current);
    box_(current, 0, 0);
    let _ = mvwprintw(current, 0, 0, "TODO");

    for (i, item) in todo.current_tasks.iter().enumerate() {
	if todo.active_window == Window::CurrentTasks && i == todo.cursor {
	    wattron(current, A_BOLD | A_UNDERLINE);
	} else {
	    wattroff(current, A_BOLD | A_UNDERLINE);
	}
	let _ = mvwprintw(current, i as i32 + 1, 1, item.get_description());
    }
    wrefresh(current);
}

fn refresh_archived(archived: WINDOW, todo: &Todo) {
    wclear(archived);
    box_(archived, 0, 0);
    let _ = mvwprintw(archived, 0, 0, "DONE");

    for (i, item) in todo.archived_tasks.iter().enumerate() {
	if todo.active_window == Window::ArchivedTasks && i == todo.cursor {
	    wattron(archived, A_BOLD | A_UNDERLINE);
	} else {
	    wattroff(archived, A_BOLD | A_UNDERLINE);
	}
	let _ = mvwprintw(archived, i as i32 + 1, 1, item.get_description());
    }
    wrefresh(archived);
}

fn refresh_help_window(help: WINDOW, dimensions: (i32, i32)) {
    box_(help, 0, 0);

    let _ = mvwprintw(help, 0, 0, "HELP");
    // 1st column
    let _ = mvwprintw(help, 1, 1, "up/down - navigate");
    let _ = mvwprintw(help, 2, 1, "TAB - change window");
    let _ = mvwprintw(help, 3, 1, "q - quit");
    // 2nd column
    let _ = mvwprintw(help, 1, dimensions.1 / 4 + 1, "d - mark as done");

    wrefresh(help);
}

const HELP_HEIGHT: i32 = 5;

fn main() {
    let root = initscr();
    keypad(initscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    refresh();

    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    getmaxyx(root, &mut max_y, &mut max_x);

    let help = newwin(HELP_HEIGHT, max_x, max_y - HELP_HEIGHT, 0);
    refresh_help_window(help, (max_y, max_x));

    let current_tasks = newwin(max_y - HELP_HEIGHT, max_x / 2, 0, 0);
    let archived_tasks = newwin(max_y - HELP_HEIGHT, max_x / 2 + 1, 0, max_x / 2);

    let mut todo = Todo::new_test();

    loop {
	refresh_current(current_tasks, &todo);
	refresh_archived(archived_tasks, &todo);
	let ch = getch();
	match ch {
	    KEY_UP => todo.cursor_up(),
	    KEY_DOWN => todo.cursor_down(),
	    9 => todo.cursor_change_window(),    // 9 is for `TAB`
	    100 => todo.mark_selected_as_done(), // 100 is for `d`
	    113 => break,                        // 113 is for `q`
	    _ => {}
	}
    }

    endwin();
}
