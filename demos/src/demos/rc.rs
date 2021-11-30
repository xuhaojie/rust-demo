use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "rc",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![DemoItem {
        title: "rc",
        execute: run,
        enable: true,
    }];
    execute_demos(&demos);
}

use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons, Nil};
fn run() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // clone 实现那样对所有数据进行深拷贝。Rc::clone 只会增加引用计数
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
