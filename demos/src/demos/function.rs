use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "function demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "demo_item_function_pointer",
            execute: demo_item_function_pointer,
            enable: true,
        },
        DemoItem {
            title: "demo_item_function_pointer_and_closure",
            execute: demo_item_function_pointer_and_closure,
            enable: true,
        },

        DemoItem {
            title: "demo_item_returns_trait_object",
            execute: demo_item_returns_trait_object,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

fn demo_item_function_pointer() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    {
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }
        let answer = do_twice(add_one, 5);

        println!("The answer is: {}", answer);
    }

    // 利用类型别名简化
    {
        type Operation = fn(i32) -> i32;
        fn do_twice(f: Operation, arg: i32) -> i32 {
            f(arg) + f(arg)
        }
        let answer = do_twice(add_one, 5);

        println!("The answer is: {}", answer);
    }
    // 不同于闭包， fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数
}
/*
函数指针实现了所有三个闭包 trait（ Fn 、 FnMut 和 FnOnce ），所以总是可以在调用期望
闭包的函数时传递函数指针作为参数。倾向于编写使用泛型和闭包 trait 的函数，这样它就能
接受函数或闭包作为参数。
高级函数与闭包
512
一个只期望接受 fn 而不接受闭包的情况的例子是与不存在闭包的外部代码交互时：C 语言
的函数可以接受函数作为参数，但 C 语言没有闭包。
 */
fn demo_item_function_pointer_and_closure() {
    // function pointer way
    {
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers
            .iter()
            .map(ToString::to_string)
            .collect();
    }
    // closure way
    {
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers
            .iter()
            .map(|i| i.to_string())
            .collect();
    }
    // 另一个实用的模式暴露了元组结构体和元组结构体枚举成员的实现细节。这些项使用 () 作
    // 为初始化语法，这看起来就像函数调用，同时它们确实被实现为返回由参数构造的实例的函数。
    // 它们也被称为实现了闭包 trait 的函数指针，并可以采用类似如下的方式调用：
    {
        enum Status {
            Value(u32),
            Stop,
        }
        let list_of_statuses: Vec<Status> =
            (0u32..20)
                .map(Status::Value)
                .collect();
        // 这里创建了 Status::Value 实例，它通过 map 用范围的每一个 u32 值调用 Status::Value的初始化函数。
        // 一些人倾向于函数风格，一些人喜欢闭包。这两种形式最终都会产生同样的代码，所以请使用对你来说更明白的形式吧。
    }
}

// 返回函数对象
fn demo_item_returns_trait_object() {
    // 闭包表现为 trait，这意味着不能直接返回闭包
    /*
    这段代码尝试直接返回闭包，它并不能编译：
    编译器给出的错误是：the trait bound `std::ops::Fn(i32) -> i32 + 'static: std::marker::Sized` is not satisfied
    Rust 并不知道需要多少空间来储存闭包。
    fn returns_closure() -> Fn(i32) -> i32 {
        |x| x + 1
    }
    */
    // 解决办法：可以使用 trait 对象：
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
