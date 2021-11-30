use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "pattern demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "demo_item_let",
            execute: demo_item_let,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for",
            execute: demo_item_for,
            enable: true,
        },
        DemoItem {
            title: "demo_item_if_let",
            execute: demo_item_if_let,
            enable: true,
        },
        DemoItem {
            title: "demo_item_while_let",
            execute: demo_item_while_let,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_with_literal",
            execute: demo_item_match_with_literal,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_with_variable_binding",
            execute: demo_item_match_with_variable_binding,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_with_multi_match",
            execute: demo_item_match_with_multi_match,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_with_range",
            execute: demo_item_match_with_range,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_with_struct",
            execute: demo_item_match_with_struct,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_with_enum",
            execute: demo_item_match_with_enum,
            enable: true,
        },
        DemoItem {
            title: "demo_item_pattern_with_reference",
            execute: demo_item_pattern_with_reference,
            enable: true,
        },
        DemoItem {
            title: "demo_item_pattern_with_tupple",
            execute: demo_item_pattern_with_tupple,
            enable: true,
        },
        DemoItem {
            title: "demo_item_function_with_ignore",
            execute: demo_item_function_with_ignore,
            enable: true,
        },
        DemoItem {
            title: "demo_item_function_with_multi_ignore",
            execute: demo_item_function_with_multi_ignore,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_with_ignore",
            execute: demo_item_match_with_ignore,
            enable: true,
        },
        DemoItem {
            title: "demo_item_ignore_unused_variable",
            execute: demo_item_ignore_unused_variable,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_ignore_rest",
            execute: demo_item_match_ignore_rest,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_guard",
            execute: demo_item_match_guard,
            enable: true,
        },
        DemoItem {
            title: "demo_item_multi_match_and_match_guard",
            execute: demo_item_multi_match_and_match_guard,
            enable: true,
        },
        DemoItem {
            title: "demo_item_match_and_at",
            execute: demo_item_match_and_at,
            enable: true,
        },
        DemoItem {
            title: "demo_it_legcy_match",
            execute: demo_it_legcy_match,
            enable: true,
        },
        DemoItem {
            title: "demo_it_legcy_match",
            execute: demo_it_legcy_match,
            enable: true,
        },
        DemoItem {
            title: "demo_item_parttern_as_function_parameter",
            execute: demo_item_parttern_as_function_parameter,
            enable: true,
        },
        DemoItem {
            title: "demo_item_coins",
            execute: demo_item_coins,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

// 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。
// 能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）。一个例子就是 let x = 5; 语句中的 x，因为 x 可以匹配任何值所以不可能会失败
// 对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable） if let Some(x) = a_value 表达式中的 Some(x)；如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配。

// let 语句   let PATTERN = EXPRESSION;
fn demo_item_let() {
    let (x, y, z) = (1, 2, 3);
    let (x, y, _) = (1, 2, 3); // 也可以使用 _ 或 ..，如 “忽略模式中的值” 部分所示

    if let x = 5 {
        // Rust 会抱怨将不可反驳模式用于 if let 是没有意义的：irrefutable `if let` pattern
        println!("{}", x);
    };
}

fn demo_item_for() {
    //for 循环
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

//if let 条件表达式
fn demo_item_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

//while let 条件循环
fn demo_item_while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// 匹配字面值(常量) literal
fn demo_item_match_with_literal() {
    let x = 1;
    // 匹配字面值 literal
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn demo_item_match_with_variable_binding() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), //一个 match 语句其中一个分支引入了覆盖变量 y
        _ => println!("Default case, x = {:?}", x),
    }
    // 一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了。
    // 最后的 println! 会打印 at the end: x = Some(5), y = 10。
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// 多个模式
fn demo_item_match_with_multi_match() {
    let x = 1;
    // 多个模式
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// 通过 ... 匹配值的范围
fn demo_item_match_with_range() {
    let x = 5;
    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn demo_item_match_with_struct() {
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn demo_item_match_with_enum() {
    // 解构枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
    }
}

// 解构引用
fn demo_item_pattern_with_reference() {
    struct Point {
        x: i32,
        y: i32,
    }
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
}

// 解构结构体和元组
fn demo_item_pattern_with_tupple() {
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    print!("{} {}", feet, inches);
}

// 使用 _ 忽略整个值
fn demo_item_function_with_ignore() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);
}

// 使用 _ 忽略整个值
fn demo_item_function_with_multi_ignore() {
    let numbers = (2, 4, 8, 16, 32);
    // 忽略元组的多个部分
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

// 使用嵌套的 _ 忽略部分值
fn demo_item_match_with_ignore() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

// 使用嵌套的 _ 忽略部分值
fn demo_item_ignore_unused_variable() {
    // 通过在名字前以一个下划线开头来忽略未使用的变量
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    //	println!("{:?}", s); // 因为 s 的值仍然会移动进 _s，并阻止我们再次使用 s
}

// 用 .. 忽略剩余值
fn demo_item_match_ignore_rest() {
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    /* 使用 .. 必须是无歧义的
    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
    */
}

// 匹配守卫（match guard）是一个指定与 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支
fn demo_item_match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// 结合多个模式与匹配守卫
fn demo_item_multi_match_and_match_guard() {
    // 结合多个模式与匹配守卫
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// @ 在模式中绑定值的同时测试它
fn demo_item_match_and_at() {
    enum HelloMessage {
        Hello { id: i32 },
    }
    let msg = HelloMessage::Hello { id: 5 };
    match msg {
        HelloMessage::Hello {
            id: id_variable @ 3...7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        HelloMessage::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        }
        HelloMessage::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

// 遗留模式： ref 和 ref mut
//你可能会在老的 Rust 代码中看到它们，所以请记住它们仍有价值。
// 在老版本的 Rust 中，match 会假设你希望移动匹配到的值。不过有时并不希望如此。例如：
fn demo_it_legcy_match() {
    let robot_name = &Some(String::from("Bors"));

    match robot_name {
        Some(name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

    match robot_name {
        &Some(ref name) => println!("Found a name: {}", name), // ref 关键字就像模式中 & 的对立面；它表明 “请将 ref 绑定到一个 &String 上，不要尝试移动”
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);
}

// 函数参数也可以是模式
fn demo_item_parttern_as_function_parameter() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location:({}, {})", x, y);
    }
    print_coordinates(&(1, 2));
    let point = (3, 5);
    print_coordinates(&point);
}

fn demo_item_coins() {
    #[derive(Debug)] // 这样可以可以立刻看到州的名称
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
