mod client;
pub use client::*;
use snarkos_node::Node;

// 实现观察者模式。
struct block {}

pub trait Observer {
    fn update();
}

pub trait Subject {
    fn NotifyObserver();
}
