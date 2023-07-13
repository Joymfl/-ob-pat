mod event;
mod observers;
mod subject;

pub use event::*;
pub use observers::*;
pub use subject::*;

fn main() {
    let mut subject1 = Generator::new(10);
    let mut observer = Observer::new(0);
    let mut observer2 = Observer::new(10);
    subject1.register(&mut observer);
    subject1.register(&mut observer2);

    subject1.update(100);
    subject1.update(200);
    subject1.update(300);
}
