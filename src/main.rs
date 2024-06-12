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
}

const HELP_WIDTH: i32 = 40;
const HELP_HEIGHT: i32 = 6;

fn main() {
    let root = initscr();

    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    let _ = getmaxyx(root, &mut max_y, &mut max_x);

    let current_tasks = newwin(max_y - HELP_HEIGHT, max_x / 2, 0, 0);
    box_(current_tasks, 0, 0);
    wrefresh(current_tasks);

    let archived_tasks = newwin(max_y - HELP_HEIGHT, max_x / 2, 0, max_x / 2);
    box_(archived_tasks, 0, 0);
    wrefresh(archived_tasks);

    let help = newwin(
	HELP_HEIGHT,
	HELP_WIDTH,
	max_y - HELP_HEIGHT,
	(max_x / 2) - (HELP_WIDTH / 2),
    );

    box_(help, 0, 0);
    let _ = mvwprintw(help, 1, 1, "HOWTO:");
    wrefresh(help);

    wgetch(help);

    endwin();
}
