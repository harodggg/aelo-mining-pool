mod client;
pub use client::*;

// 实现观察者模式。
struct block {}

pub trait Observer {
    fn update();
}

pub trait Subject {
    fn NotifyObserver();
}
