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
    let demos = vec![
        DemoItem {
            title: "threads",
            execute: run,
            enable: true,
        },
        DemoItem {
            title: "thread_demo2",
            execute: thread_demo2,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

use std::thread;
use std::time::Duration;

fn download(url: &str) {
    println!("{}", url);
}

fn get_two_sites() {
    let thread1 = thread::spawn(|| download("https://www.foo.com"));
    let thread2 = thread::spawn(|| download("https://www.foo.com"));
    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn run() {
    let t = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    t.join().unwrap();

    get_two_sites();
}

fn thread_demo2() {
    let t = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    t.join().unwrap();
}
