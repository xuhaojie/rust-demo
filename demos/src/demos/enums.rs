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
			title: "demo_item_c_style_enum",
			execute: demo_item_c_style_enum,
			enable: true,
		},
		DemoItem {
			title: "demo_item_enum_with_value",
			execute: demo_item_enum_with_value,
			enable: true,
		},
		DemoItem {
			title: "demo_item_enum_with_method",
			execute: demo_item_enum_with_method,
			enable: true,
		},
		DemoItem {
			title: "demo_item_enum_option",
			execute: demo_item_enum_option,
			enable: true,
		},
		DemoItem {
			title: "demo_item_enum_complex",
			execute: demo_item_enum_complex,
			enable: true,
		},
	];
	execute_demos(&demos);
}
fn demo_item_c_style_enum() {
	#[derive(PartialEq)]
	enum IpAddrKind {
		V4,
		V6,
	}
	struct IpAddr {
		kind: IpAddrKind,
		address: String,
	}
	let home = IpAddr {
		kind: IpAddrKind::V4,
		address: String::from("127.0.0.1"),
	};
	let loopback = IpAddr {
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	};
	fn displayIpAddress(addr: &IpAddr) {
		if addr.kind == IpAddrKind::V4 {
			println!("IP v4 address: {}", addr.address);
		} else {
			println!("IP v6 address: {}", addr.address);
		}
	}
	displayIpAddress(&home);
	displayIpAddress(&loopback);
}

fn demo_item_enum_with_value() {
	enum IpAddr {
		V4(u8, u8, u8, u8),
		V6(String),
	}
	let home = IpAddr::V4(127, 0, 0, 1);
	let loopback = IpAddr::V6(String::from("::1"));
	match home {
		IpAddr::V4(a, b, c, d) => println!("Ip v4 {}.{}.{}.{}", a, b, c, d),
		IpAddr::V6(addr) => println!("Ip v6 {}", addr),
	}
}

fn demo_item_enum_with_method() {
	enum Message {
		Quit,
		Move { x: i32, y: i32 },
		Write(String),
		ChangeColor(i32, i32, i32),
	}
	impl Message {
		fn call(&self) {
			match self { // &self也可以，但是没有必要
				Message::Quit => println!("Quit"),
				//Message::Move { a, b } => println!("Move to ({},{})", a, b), // variant `demo_item_enum_with_method::Message::Move` does not have these fields
				Message::Move { x, y } => println!("Move to ({},{})", x, y),
				Message::Write(msg) => println!("Write with {}", msg),
				Message::ChangeColor(r, g, b) => println!("Change color to RGB({},{},{})", r, g, b),
			}
		}
	}
	let m = Message::Write(String::from("hello"));
	m.call();
	Message::ChangeColor(0, 255, 255).call();
}

fn demo_item_enum_option() {
	let _some_number = Some(5);
	let _some_string = Some("a string");
	// 如果使用 None 而不是 Some ，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
	let _absent_number: Option<i32> = None;
}

fn demo_item_enum_complex() {
	// 需要 `allow` 来消除警告，因为只使用了枚举类型的一种取值。
	#[allow(dead_code)]
	enum Color {
		// 这三个取值仅由它们的名字（而非类型）来指定。
		Red,
		Blue,
		Green,
		// 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
		RGB(u32, u32, u32),
		HSV(u32, u32, u32),
		HSL(u32, u32, u32),
		CMY(u32, u32, u32),
		CMYK(u32, u32, u32, u32),
	}

	let color = Color::RGB(122, 17, 40);
	// 试一试 ^ 将不同的值赋给 `color`

	println!("What color is it?");
	// 可以使用 `match` 来解构 `enum`。
	match color {
		Color::Red => println!("The color is Red!"),
		Color::Blue => println!("The color is Blue!"),
		Color::Green => println!("The color is Green!"),
		Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
		Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
		Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
		Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
		Color::CMYK(c, m, y, k) => println!(
			"Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
			c, m, y, k
		),
		// 不需要其它分支，因为所有的情形都已覆盖
	}
}
