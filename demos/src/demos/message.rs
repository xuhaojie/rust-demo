use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "message demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![DemoItem {
        title: "demo_item_mpsc_message",
        execute: demo_item_mpsc_message,
        enable: true,
    }];
    execute_demos(&demos);
}

use std::sync::mpsc;
use std::thread;
use std::time::Duration; // mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）

fn demo_item_mpsc_message() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    std::thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        //println!("val = {}", val); // value borrowed here after move
    });
    std::thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        //println!("val = {}", val); // value borrowed here after move
    });

    for received in rx {
        println!("Got: {}", received);
    }
    //let received = rx.recv().unwrap(); // recv方法会阻塞主线程执行直到从通道中接收一个值
}
