pub struct Event {
    pub change: u32,
}

impl Event {
    pub fn new(change: u32) -> Self {
        Self { change }
    }
}
