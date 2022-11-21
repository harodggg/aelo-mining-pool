mod node_type;
mod traits;
use std::alloc;
use std::str;
use std::string;
use std::usize;

// 实现观察者模式。

struct block {}

pub trait Observer {
    fn update();
}

pub trait Subject {
    fn NotifyObserver();
}
