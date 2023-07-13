pub struct Observer {
    pub data: u32,
}

pub trait Observable {
    fn update(&mut self, data: u32);
}

impl Observable for Observer {
    fn update(&mut self, data: u32) {
        self.data = data;
        println!("{}", self.data);
    }
}
impl Observer {
    pub fn new(data: u32) -> Self {
        Self { data }
    }
}
