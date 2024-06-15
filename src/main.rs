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

struct Todo {
    pub current_tasks: Vec<Entry>,
    archived_tasks: Vec<Entry>,
    pub cursor: usize,
}

impl Todo {
    fn add_task(&mut self, new_entry: Entry) {
	self.current_tasks.push(new_entry);
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
    let _ = mvwprintw(help, 1, 1, "q - quit");
    let _ = mvwprintw(help, 2, 1, "BAR");
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
    };
    todo.add_task(Entry::new("first task".to_string()));
    todo.add_task(Entry::new("second task".to_string()));
    todo.add_task(Entry::new("third task".to_string()));

    //keypad(stdscr(), true);
    //keypad(archived_tasks, true);
    loop {
	// TODO: break condition
	refresh_current(current_tasks, &todo);
	refresh_archived(archived_tasks, &todo);
	let ch = getch();
	match ch {
	    // 113 is for `q`
	    113 => {
		break;
	    }
	    _ => {},
	}
    }

    endwin();
}
