use crate::DemoItem;
use crate::execute_demos;
use utility;

pub fn new() -> DemoItem {
	DemoItem {
		title: "arc demos",
		execute: execute,
		enable: true,
	}
}

fn execute() {
	let demos = vec![
		DemoItem {
			title: "demo_item_arc_basic_usage",
			execute: demo_item_arc_basic_usage,
			enable: true,
		},
	];
	execute_demos(&demos);
}

use std::sync::{Arc, Mutex};
use std::thread;
// 原子引用计数 Arc<T>
fn demo_item_arc_basic_usage() {
	let counter = Arc::new(Mutex::new(0));
	let mut handles = vec![];

	for _ in 0..10 {
		let counter = Arc::clone(&counter);
		let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num += 1;
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}
    println!("Result: {}", *counter.lock().unwrap());
}
