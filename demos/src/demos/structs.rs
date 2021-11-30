use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
	DemoItem {
		title: "struct demos",
		execute: execute,
		enable: true,
	}
}

fn execute() {
	let demos = vec![
		DemoItem {
			title: "struct_define_and_initilize",
			execute: struct_define_and_initilize,
			enable: true,
		},
		DemoItem {
			title: "access_struct_filed",
			execute: access_struct_filed,
			enable: true,
		},
		DemoItem {
			title: "Struct Field Init Shorthand",
			execute: struct_field_init_shorthand,
			enable: true,
		},
		DemoItem {
			title: "associated_functions",
			execute: associated_functions,
			enable: true,
		},
		DemoItem {
			title: "multi_impl_block",
			execute: multi_impl_block,
			enable: true,
		},
		DemoItem {
			title: "struct_update_syntax",
			execute: struct_update_syntax,
			enable: true,
		},
		DemoItem {
			title: "struct_default_value_with_update_syntax",
			execute: struct_default_value_with_update_syntax,
			enable: true,
		},
		DemoItem {
			title: "tupple_struct",
			execute: tupple_struct,
			enable: true,
		},
		DemoItem {
			title: "unit_like_struct",
			execute: unit_like_struct,
			enable: true,
		},
		DemoItem {
			title: "initilize_with_array_element",
			execute: initilize_with_array_element,
			enable: true,
		},
		DemoItem {
			title: "automatic_referencing_and_dereferencing",
			execute: automatic_referencing_and_dereferencing,
			enable: true,
		},
	];
	execute_demos(&demos);
}

#[derive(Debug)]
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

// struct define and initilize
fn struct_define_and_initilize() {
	let user = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};
	println!("user = {:#?}", user);
}

// access_struct_filed
fn access_struct_filed() {
	let mut user = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};
	user.email = String::from("anotheremail@example.com");
	println!("user = {:#?}", user);
}

// Using the Field Init Shorthand when Variables and Fields Have the Same Name
fn struct_field_init_shorthand() {
	let email = String::from("user@company.com");
	let username = String::from("jack");
	let user = User {
		//email: email,
		//username: username,
		email,    // 变量与字段同名时的字段初始化简写语法
		username, // 变量与字段同名时的字段初始化简写语法
		active: true,
		sign_in_count: 1,
	};
	println!("user = {:#?}", user);
}

// struct update syntax
fn struct_update_syntax() {
	let mut user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	let user2 = User {
		email: String::from("another@example.com"),
		username: String::from("anotherusername567"),
		//active: user1.active,
		//sign_in_count: user1.sign_in_count,
		..user1 // 使用结构体更新语法struct update syntax
	};
	println!("user2 = {:#?}", user2);
}

fn struct_default_value_with_update_syntax() {
	#[derive(Debug)]
	struct PacketHead {
		magic: u16,
		pkt_type: u16,
		length: u16,
	}
	let default_packet_head = PacketHead {
		magic: 0x55AA,
		pkt_type: 0x8100,
		length: 0,
	};
	let packet_head = PacketHead {
		..default_packet_head // 使用结构体更新语法实现类似默认值
	};
	println!("packetHead = {:#?}", packet_head);
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn associated_functions() {
	impl Rectangle {
		// 关联函数 associated functions, 相当于C++的静态成员函数
		fn square(size: u32) -> Rectangle {
			Rectangle {
				width: size,
				height: size,
			}
		}
	}

	let square = Rectangle::square(32);
	println!("square = {:#?}", square);
}

fn multi_impl_block(){
	// 这里没有理由将这些方法分散在多个 impl 块中，只是演示语法
	#[derive(Debug)]
	struct Rectangle {
		width: u32,
		height: u32,
	}

	impl Rectangle {
		fn area(&self) -> u32 {
			self.width * self.height
		}
	}
	impl Rectangle {
		fn can_hold(&self, other: &Rectangle) -> bool {
			self.width > other.width && self.height > other.height
		}
	}

}

fn tupple_struct() {
	// 没有命名字段的元组结构体
	#[derive(Debug)]
	struct Color(i32, i32, i32);
	#[derive(Debug)]
	struct Point(i32, i32, i32);

	let black = Color(0, 0, 0);
	let orign = Point(0, 0, 0);
	println!("black = {:?}", black);
	println!("orign = {:?}", orign);
}

fn unit_like_struct() {
	// 常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
	struct unit {}; // 没有任何字段的类单元结构体 unit-like structs

	trait Test {
		fn test(&self);
	}
	impl Test for unit {
		fn test(&self) {
			println!("unit::test");
		}
	}
	let t = unit {};
	t.test();
}

fn initilize_with_array_element() {
	#[derive(Debug)]
	struct Histogram {
		sum: u32,
		bins: [u32; 256],
	}

	impl Default for Histogram {
		fn default() -> Histogram {
			Histogram {
				sum: 0,
				bins: [0; 256],
			}
		}
	}
	let h: Histogram = Default::default();
	println!("{:?}", h);
}

// automatic referencing and dereferencing
fn automatic_referencing_and_dereferencing() {
	impl Rectangle {
		fn area(&self) -> u32 {
			self.width * self.height
		}
	}

	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!("rect1 is {:?}", rect1);
	// 自动引用和解引用（automatic referencing and dereferencing）
	// 当使用 object.something() 调用方法时，Rust 会自动为 object 添加 & 、 &mut 或 * 以便使 object 与方法签名匹配
	println!(
		"The area of the rectangle is {} square pixels.",
		rect1.area()
	);
	println!(
		"The area of the rectangle is {} square pixels.",
		&rect1.area()
	);
}
