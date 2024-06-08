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
    let e = Entry::new(String::from("foo"));
    println!("Entry = {:?}", e);
}
