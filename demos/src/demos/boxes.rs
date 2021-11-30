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
            title: "box_basic_usage",
            execute: box_basic_usage,
            enable: true,
        },
        DemoItem {
            title: "list_with_box",
            execute: list_with_box,
            enable: true,
        },
        DemoItem {
            title: "my_box",
            execute: my_box,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

fn box_basic_usage() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn list_with_box() {
    /*
    enum List { // recursive type has infinite size ,  insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
        Cons(i32, List),
        Nil,
    }
    */
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil}; // 为了避免每次都写List::
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn my_box() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }
    // 通过解引用运算符追踪指针的值
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    //	assert_eq!(5, y); // no implementation for `{integer} == &{integer}` the trait `PartialEq<&{integer}>` is not implemented for `{integer}`

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    //assert_eq!(5, y); // no implementation for `{integer} == Box<{integer}>`  the trait `PartialEq<Box<{integer}>>` is not implemented for `{integer}`
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 如果不实现Deref trait会报错: error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
    assert_eq!(5, *(y.deref())); // Rust 事实上在底层运行了如下代码

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]); // 如果 Rust 没有实现解引用强制多态，为了使用 &MyBox<String> 类型的值调用 hello，则不得不编写示例
                      /*
                      Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：
                      当 T: Deref<Target=U> 时从 &T 到 &U。
                      当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
                      当 T: Deref<Target=U> 时从 &mut T 到 &U。
                      */
}
