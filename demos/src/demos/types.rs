use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
	DemoItem {
		title: "enums demos",
		execute: execute,
		enable: true,
	}
}

fn execute() {
	let demos = vec![

		DemoItem {
			title: "demo_item_type_alias_basic_usage",
			execute: demo_item_type_alias_basic_usage,
			enable: true,
		},
		DemoItem {
			title: "demo_item_write_shorter_code_with_type_alias",
			execute: demo_item_write_shorter_code_with_type_alias,
			enable: true,
		},
		DemoItem {
			title: "demo_item_type_alias_with_generic",
			execute: demo_item_type_alias_with_generic,
			enable: true,
		},
		DemoItem {
			title: "demo_item_never_type_and_panic",
			execute: demo_item_never_type_and_panic,
			enable: true,
		},
		DemoItem {
			title: "demo_item_never_type_and_loop",
			execute: demo_item_never_type_and_loop,
			enable: true,
		},
		DemoItem {
			title: "demo_item_dynamically_sized_types",
			execute: demo_item_dynamically_sized_types,
			enable: true,
		},
	];
	execute_demos(&demos);
}

fn demo_item_type_alias_basic_usage(){
	type Kilometers = i32;
	let x: i32 = 5;
	let y: Kilometers = 5;
	println!("x + y = {}", x + y);

}


fn demo_item_write_shorter_code_with_type_alias(){
	// 在函数签名或类型注解中每次都书写这个类型将是枯燥且易于出错的
	fn without_type_alias(){
		let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

		fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
			// --snip--
		}
		fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
			// --snip--
			Box::new(|| ())
		}
	}

	//  引入类型别名来减少重复
	fn with_type_alias(){
		type Thunk = Box<dyn Fn() + Send + 'static>;
		let f: Thunk = Box::new(|| println!("hi"));
		fn takes_long_type(f: Thunk) {
			// --snip--
		}
		fn returns_long_type() -> Thunk {
			// --snip--
			Box::new(|| ())
		}
	}
}

fn demo_item_type_alias_with_generic(){

	fn without_type_alias(){
		use std::io::Error;
		use std::fmt;
		pub trait Write {
			fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
			fn flush(&mut self) -> Result<(), Error>;
			fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
			fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
		}
	}
	// 使用类型别名后
	fn with_type_alias(){
		use std::io::Error;
		use std::fmt;
		type Result<T> = std::result::Result<T, std::io::Error>;
		pub trait Write {
			fn write(&mut self, buf: &[u8]) -> Result<usize>;
			fn flush(&mut self) -> Result<()>;
			fn write_all(&mut self, buf: &[u8]) -> Result<()>;
			fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
		}
	}
}

// 从不返回的 never type, 而从不返回的函数被称为 发散函数（diverging functions）
fn demo_item_never_type(){
	// Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。
	// 我们更倾向于称之为 never type。这个名字描述了它的作用：在函数从不返回的时候充当返回值。
	/*
	fn bar() -> ! {
		// --snip--
	}
	*/

	// 如下代码不能工作,这里的 guess 必须既是整型 也是 字符串，而 Rust 要求 guess 只能是一个类型
	/*
	fn not_work(){
		let guess = "3";
		let guess = match guess.trim().parse() {
			Ok(_) => 5,
			Err(_) => "hello",
		};
	}
	*/
	// 那么continue 返回了什么呢？正如你可能猜到的， continue 的值是 ! 。
	// 当 Rust 要计算 guess 的类型时，它查看这两个分支。前者是 u32 值，而后者是 ! 值。
	// 因为 ! 并没有一个值，Rust 决定guess 的类型是 u32
	let guess = "3";
	loop {
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		break;
	}
}

fn demo_item_never_type_and_panic(){
	enum Option<T> {
		Some(T),
		None,
	}
	impl<T>Option<T> {
		pub fn unwrap(self) -> T {
			match self {
				Option::Some(val) => val,
				Option::None => panic!("called `Option::unwrap()` on a `None` value"),
				/*
				Rust 知道 val 是 T 类型， panic! 是! 类型，所以整个 match 表达式的结果是 T 类型。
				这能工作是因为 panic! 并不产生一个值；它会终止程序。对于 None 的情况， unwrap 并不返回一个值，所以这些代码是有效。
				*/
			}
		}
	}
}
// 最后一个有着 ! 类型的表达式是 loop ：
fn demo_item_never_type_and_loop(){
	print!("forever ");
	loop {
		print!("and ever ");
		break;
	}
}

// 动态大小类型(（dynamically sized types 或 unsized types)和 Sized trait，必须将动态大小类型的值置于某种指针之后。
fn demo_item_dynamically_sized_types(){
	{
		fn generic<T>(t: T) {
			// --snip--
		}

	}
	// 实际上被当作如下处理：
	{
		fn generic<T: Sized>(t: T) {
			// --snip--
		}
	}
	// 泛型函数默认只能用于在编译时已知大小的类型。然而可以使用如下特殊语法来放宽这个限制：
	{
		fn generic<T: ?Sized>(t: &T) { // 另外注意我们将 t 参数的类型从 T 变为了 &T ：因为其类型可能不是 Sized 的
			// --snip--
		}
	}

}