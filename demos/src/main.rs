mod demos;
use crate::demos::*;
use utility;

type Operation = fn();

pub struct DemoItem {
    title: &'static str,
    execute: Operation,
    enable: bool,
}

pub fn execute_demos(demos: &Vec<DemoItem>) {
    for d in demos.iter() {
        if d.enable {
            utility::centre_print(d.title, '-', 80);
            println!();
            (d.execute)();
            println!();
        }
    }
}

fn main() {
    let demos = vec![
        (false, arc::new()),
        (false, array::new()),
        (true, asyncs::new()),
        (false, boxes::new()),
        (false,  closure::new()),
//        (false, cust_derive::new()),
        (false, drop::new()),
        (false, enums::new()),
        (false, error::new()),
        (false, function::new()),
        (false, generics::new()),
        (false, iteration::new()),
        (false, iterator::new()),
        (false, map::new()),
        (false, marco::new()),
        (false, message::new()),
        (false, pattern::new()),
        (false, rc::new()),
        (false, refcell::new()),
        (false, string::new()),
        (false, structs::new()),
        (false, thread::new()),
        (false, template::new()),
        (false, traits::new()),
        (false, types::new()),
        (false, unsafes::new()),
        (false, vector::new()),
    ];

    for d in demos.iter() {
        if d.0 && d.1.enable {
            utility::centre_print(d.1.title, '=', 80);
            println!();
            (d.1.execute)();
            println!();
        }
    }
}

// 用trait实现太啰嗦了
fn trait_way() {
    pub trait Demo {
        fn title(&self) -> &str;
        fn execute(&self);
    }
    pub struct StructDemo {}

    impl StructDemo {
        pub fn new() -> Box<dyn Demo> {
            Box::new(StructDemo {})
        }
    }

    impl Demo for StructDemo {
        fn title(&self) -> &str {
            "sdfafda"
        }
        fn execute(&self) {}
    }

    let demos: Vec<Box<dyn Demo>> = vec![StructDemo::new()];

    for d in demos.iter() {
        println!("{}", d.title());
        d.execute();
    }
}
