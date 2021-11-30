use crate::execute_demos;
use crate::DemoItem;
use std::collections::HashSet;

pub fn new() -> DemoItem {
    DemoItem {
        title: "closure demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "demo_item_basic_usage",
            execute: demo_item_basic_usage,
            enable: true,
        },
        DemoItem {
            title: "demo_item_capture_mode_immutable_borrow",
            execute: demo_item_capture_mode_immutable_borrow,
            enable: true,
        },

        DemoItem {
            title: "demo_item_lazy_calculation",
            execute: demo_item_lazy_calculation,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

/*
有些语言中没有 closure 和普通函数的区分，但 Rust 有。对 Rust 来说普通函数就是一段代码。
而 closure 和 C++ 类似：每个 closure 会创建一个匿名的struct，编译器会在当前上下文捕获 closure 代码中的外部变量然后塞进这个结构体里面。
这件事非常重要，请默念三遍一个 closure 就是一个捕获了当前上下文变量的结构体（外加一段代码，这不重要）。
*/
/*
let m = 1.0;
let c = 2.0;

let line = |x| m*x + c;

// 等价于

struct SomeUnknownType<'a> {
    m: &'a f64,
    c: &'a f64
}

impl<'a> SomeUnknownType<'a> {
    fn call(&self, x: f64) -> f64 {
        self.m * x + self.c
    }
}
*/
use std::thread;
use std::time::Duration;
fn demo_item_basic_usage() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1; //  consider giving this closure parameter a type , 加上调用就好了
    let add_one_v4 = |x| x + 1;
    let x = add_one_v3(3);
    let x = add_one_v4(3);
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    //let n = example_closure(5); // expected struct `String`, found integer
}

/*
复合类型（structs，tuples，enums）总是被整体捕获，而不是根据每个字段独立捕获，所以为了捕获单个字段（field），可以将该字段先借用到一个本地变量，比如：
*/
fn demo_item_capture_mode_immutable_borrow() {
    struct SetVec {
        set: HashSet<u32>,
        vec: Vec<u32>,
    }

    impl SetVec {
        fn populate(&mut self) {
            let vec = &mut self.vec;
            self.set.iter().for_each(|&n| {
                vec.push(n);
            })
        }
    }
}

fn demo_item_lazy_calculation(){

    struct Cacher<T>
        where
            T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        //为闭包的参数和返回值增加可选的类型注解
        let expensive_closure = |num: u32| -> u32 {
            //let expensive_closure = |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }

        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;

        //	println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

