mod client;

mod node_interface;
//use node_interface::*;

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
