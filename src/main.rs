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

    let w = newwin(
	HELP_HEIGHT,
	HELP_WIDTH,
	max_y - HELP_HEIGHT,
	(max_x / 2) - (HELP_WIDTH / 2),
    );

    box_(w, 0, 0);
    mvwprintw(w, 1, 1, "HOWTO:");
    mvwprintw(w, 2, 1, "FOO");
    mvwprintw(w, 3, 1, "FOO");
    mvwprintw(w, 4, 1, "FOO");
    wgetch(w);

    endwin();
}
