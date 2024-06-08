#[derive(Debug)]
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
}

fn main() {
    let mut todo_list = vec![];
    todo_list.push(Entry::new(String::from("foo")));
    todo_list.push(Entry::new(String::from("bar")));

    for entry in todo_list {
	println!("Entry = {:?}", entry);
    }
}
