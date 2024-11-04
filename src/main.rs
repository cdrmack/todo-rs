extern crate ncurses;
use ncurses::*;

pub mod todo;
pub mod entry;

use todo::*;

#[derive(PartialEq)]
pub enum Window {
    CurrentTasks,
    ArchivedTasks,
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
