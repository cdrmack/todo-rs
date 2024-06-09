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
	Entry { description, state: EntryState::Todo }
    }

    fn change_state(&mut self, new_state: EntryState) {
	if self.state != new_state {
	    self.state = new_state;
	}
    }
}

fn main() {
    let mut todo_list = vec![];
    todo_list.push(Entry::new(String::from("foo")));
    todo_list.push(Entry::new(String::from("bar")));

    for entry in todo_list.iter() {
	println!("Entry = {:?}", entry);
    }

    todo_list[1].change_state(EntryState::InProgress);

    for entry in todo_list.iter() {
	println!("Entry = {:?}", entry);
    }
}
