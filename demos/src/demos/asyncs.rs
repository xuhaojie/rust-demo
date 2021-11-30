use crate::DemoItem;
use crate::execute_demos;
use futures::executor::block_on;
use futures::join;

pub fn new() -> DemoItem {
	DemoItem {
		title: "await demos",
		execute: execute,
		enable: true,
	}
}

fn execute() {
	let demos = vec![
        DemoItem {
			title: "demo_item_block_on",
			execute: demo_item_block_on,
			enable: true,
		},
		DemoItem {
			title: "demo_item_future",
			execute: demo_item_future,
			enable: true,
		},
		DemoItem {
			title: "demo_item_prototype",
			execute: demo_item_prototype,
			enable: true,
		},
		DemoItem {
			title: "demo_item_",
			execute: demo_item_,
			enable: true,
		},

/*
		DemoItem {
			title: "demo_item_async_io",
			execute: demo_item_async_io,
			enable: true,
		},		
*/
	];
	execute_demos(&demos);
}

fn demo_item_block_on(){
	async fn say_hi() {
		println!("hi");
	}
	/*
	use futures::Future;
	fn say_hi() -> impl Future<Output = ()> {
		println!("hi");
	}
	*/
	let future = say_hi();
	block_on(future);
}

fn demo_item_prototype(){
	async fn foo() -> u8 { 5 }
	fn bar() -> impl futures::Future<Output = u8> {
		let futures = async {
			let x: u8 = foo().await;
			x+5
		};
		futures
	}
	bar();
	println!("done");
}

fn demo_item_future(){
	async fn learn_song(){
		println!("learn_song");
	}
	async fn sing_song(){
		println!("sing_song");
	}
	async fn dance(){
		println!("dance")
	}
	async fn learn_and_sing(){
		let song = learn_song().await;
		sing_song().await;
	}
	async fn async_main(){
		let f1 = learn_and_sing();
		let f2 = dance();
		futures::join!(f1,f2);
	}
	block_on(async_main());
	println!("done");
}

fn demo_item_async_io(){
	use async_std::io;
	use async_std::fs::File;
	use async_std::prelude::*;
	async fn read_file(path: &str) -> io::Result<String> {
		let mut file = File::open(path).await?;
		let mut buffer = String::new();
		file.read_to_string(&mut buffer).await?;
		Ok(buffer)
	}

	async fn load_file(path: &str){
		let r1 = read_file(path).await;
		println!("{} size:{}",path, r1.unwrap().len());
	}

	async fn load_files(){
		join!(load_file("../../demos/src/demos/array.rs"), load_file("../../demos/src/demos/arc.rs"));
	}

	block_on(load_files());
}
fn demo_item_(){
	use async_std::task::{sleep, spawn};
	use std::time::Duration;
	async fn sleep_us(){
		for i in 1..=10 {
			println!("Sloop_us {}", i);
			sleep(Duration::from_millis(500)).await;
		}
	}

	async fn interrupt_us() {
		for i in 1..=5 {
			println!("Interrupt_us {}", i);
			sleep(Duration::from_millis(1000)).await;
		}
	}
	// 手工实现
	fn sleepus() -> impl std::future::Future<Output=()> {
		async {
			for i in 1..=10 {
				println!("Sleepus {}", i);
				sleep(Duration::from_millis(500)).await;
			}
		}
	}

	async fn work(){
		let sleep_us = spawn(sleep_us());
		interrupt_us().await;
		sleep_us.await;
		sleepus().await;
	}
	block_on(work());
}
