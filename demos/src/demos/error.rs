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
            title: "demo_item_handle_error_with_match",
            execute: demo_item_handle_error_with_match,
            enable: true,
        },
        DemoItem {
            title: "demo_item_handle_different_error_with_math",
            execute: demo_item_handle_different_error_with_math,
            enable: true,
        },
        DemoItem {
            title: "demo_item_handle_different_error_with_closure",
            execute: demo_item_handle_different_error_with_closure,
            enable: true,
        },
        DemoItem {
            title: "demo_item_simple_panic_process_with_unwrap_or_expect",
            execute: demo_item_simple_panic_process_with_unwrap_or_expect,
            enable: true,
        },

        DemoItem {
            title: "demo_item_return_result_with_match",
            execute: demo_item_return_result_with_match,
            enable: true,
        },
        DemoItem {
            title: "demo_item_return_result_with_question_operation",
            execute: demo_item_return_result_with_question_operation,
            enable: true,
        },
        DemoItem {
            title: "demo_item_chaining_with_question_operation",
            execute: demo_item_chaining_with_question_operation,
            enable: true,
        },
        DemoItem {
            title: "demo_item_use_fs_read_to_string",
            execute: demo_item_use_fs_read_to_string,
            enable: true,
        },
        DemoItem {
            title: "demo_item_main_function_with_result",
            execute: demo_item_main_function_with_result,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

// 使用 match 表达式处理可能会返回的 Result 成员
fn demo_item_handle_error_with_match(){
    use std::fs::File;
    let file_name = "hello.txt";
    let f = File::open(file_name);
    match f {
        Ok(file) =>  println!("File opened."),
        Err(error) => {
            println!("Problem opening the file {} : {:?}", file_name, error)
        }
    };
    // panic 的处理方式，不适合用于演示
    fn demo() {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening the file: {:?}", error)
            },
        };
    }
}

// 使用match处理不同类型的错误
fn demo_item_handle_different_error_with_math(){
    use std::fs::File;
    use std::io::ErrorKind;
    let file_name = "hello.txt";
    let f = File::open(file_name);
    match f {
        Ok(file) => println!("File opened."),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => println!("File created "),
                Err(e) => println!("Problem creating the file: {:?}", e),
            },
            other_error => println!("Problem opening the file: {:?}", other_error),
        },
    };
}

// 使用closure处理不同类型的错误
fn demo_item_handle_different_error_with_closure() {
    use std::fs::File;
    use std::io::ErrorKind;
    let file_name = "hello.txt";
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
              File::create("hello.txt").unwrap_or_else(|error| {
                  panic!("Problem creating the file: {:?}", error);
              })
        } else {
              panic!("Problem opening the file:{:?}", error);
        }
    });
}
// 失败时 panic 的简写： unwrap 和 expect
fn demo_item_simple_panic_process_with_unwrap_or_expect(){
    use std::fs::File;
    let file_name = "hello.txt";
    // 失败时 panic 的简写
    let f = File::open(file_name).unwrap();
    // expect 提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源
    let f = File::open(file_name).expect("Failed to open hello.txt");
}

// 使用 match 将错误返回给代码调用者
fn demo_item_return_result_with_match(){
    use std::io;
    use std::io::Read;
    use std::fs::File;
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

// 传播错误的简写： ? 运算符
fn demo_item_return_result_with_question_operation(){
    use std::io;
    use std::io::Read;
    use std::fs::File;
    fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
        let mut f = File::open(file_name)?; // 使用 ? 运算符向调用者返回错误
        let mut s = String::new();
        f.read_to_string(&mut s)?; // 使用 ? 运算符向调用者返回错误的函数
        Ok(s)
    }
    read_username_from_file("hello.txt");
}

// 问号运算符之后的链式方法调用
fn demo_item_chaining_with_question_operation(){
    use std::io;
    use std::io::Read;
    use std::fs::File;
    fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
        let mut s = String::new();
        File::open(file_name)?.read_to_string(&mut s)?; // 使用 ? 运算符向调用者返回错误的函数
        Ok(s)
    }
    read_username_from_file("hello.txt");
}

// 使用 fs::read_to_string简化代码
fn demo_item_use_fs_read_to_string(){
    use std::io;
    use std::fs;
    fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
        fs::read_to_string(file_name)
    }
    read_username_from_file("hello.txt");
}

fn demo_item_read_file4(){
    use std::io;
    use std::fs;
    fn read_username_from_file4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}

//  main 函数的一个有效的返回值是() ，同时出于方便，另一个有效的返回值是 Result<T, E> ，如下所示：
fn demo_item_main_function_with_result(){
    use std::error::Error;
    use std::fs::File;
/*
main 函数是特殊的，其必须返回什么类型是有限制的。 main 函数的一个有效的返回值是() ，
同时出于方便，另一个有效的返回值是 Result<T, E> ，如下所示：
*/
    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;
        Ok(())
    }
    main();
}

