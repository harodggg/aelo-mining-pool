#[derive(Debug, Default)]
pub struct BlockObserver {}

pub trait BlockObservable {
    fn add_observer(&self);
    fn remove_observer(&self);
}


impl BlockObservable for BlockObserver {
    fn add_observer(&self) {}
    fn remove_observer(&self) {}
}
