use crate::{Event, Observable, Observer};

pub struct Generator<'a> {
    pub state: u32,
    pub listeners: Vec<&'a mut Observer>,
}

pub trait Generate<'a> {
    fn register(&mut self, observer: &'a mut Observer);
    fn notify(&mut self, event: Event);
}

impl<'a> Generate<'a> for Generator<'a> {
    fn register(&mut self, observer: &'a mut Observer) {
        self.listeners.push(observer);
    }
    fn notify(&mut self, event: Event) {
        for listener in &mut self.listeners {
            listener.update(event.change);
        }
    }
}

impl<'a> Generator<'a> {
    pub fn new(data: u32) -> Self {
        Self {
            state: data,
            listeners: Vec::new(),
        }
    }
    pub fn update(&mut self, data: u32) {
        self.state = data;
        let event = Event { change: data };
        self.notify(event);
    }
}
