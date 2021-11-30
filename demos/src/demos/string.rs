use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "string demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "string_initalize",
            execute: string_initalize,
            enable: true,
        },
        DemoItem {
            title: "append_string",
            execute: append_string,
            enable: true,
        },
        DemoItem {
            title: "join_string",
            execute: join_string,
            enable: true,
        },
        DemoItem {
            title: "format_string",
            execute: format_string,
            enable: true,
        },
        DemoItem {
            title: "unicode_string",
            execute: unicode_string,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

fn string_initalize() {
    let mut s = String::new();
    let data = "initial contests";
    let s = data.to_string();
    let s = "initial contests".to_string();
    let s = String::from("initial contents");
    let hello = String::from("你好");
    println!("{}", hello);
}

// 使用 push_str 和 push 附加字符串
fn append_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 =  {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s = {}", s);
}
// 使用 + 运算符拼接字符串
fn join_string() {
    // 使用 + 运算符或 format! 宏拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    //let s3 = s1 + & s2;  // 注意 s1 被移动了，不能继续使用
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
                       //println!("s1 = {} ,s3={}",s1, s3); // ^^ value borrowed here after move
    println!("s3 = {}", s3);
}

// 使用format! 宏拼接字符串
fn format_string() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
}

fn unicode_string() {
    let hello = String::from("你好");
    let s1 = String::from("你好");
    // let h = s1[0]; 不支持
    let len = s1.len();
    let s = &hello[0..3];
    // let s = &hello[0..2];  //panic
    for c in "你好吗".chars() {
        println!("{}", c);
    }

    for b in "नमİते".bytes() {
        println!("{}", b);
    }
}
