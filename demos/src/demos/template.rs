use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "thread demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![DemoItem {
        title: "threads",
        execute: run,
        enable: true,
    }];
    execute_demos(&demos);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    //fn largest<T> (list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn run() {
    let number_list = vec![1, 2, 23, 34, 8, 100];
    let max_number = largest(&number_list);
    println!("max_number = {}", max_number);
    let p = Point { x: 100, y: 200 };
    println!("{:#?}", p);
}
