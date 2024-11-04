#[derive(Debug, PartialEq)]
pub enum EntryState {
    Todo,
    Done,
}

#[derive(Debug)]
pub struct Entry {
    description: String,
    state: EntryState,
}

impl Entry {
    pub fn new(description: String) -> Self {
        Entry {
            description,
            state: EntryState::Todo,
        }
    }

    pub fn change_state(&mut self, new_state: EntryState) {
        if self.state != new_state {
            self.state = new_state;
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
