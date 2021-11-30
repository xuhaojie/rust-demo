use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "array demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "array_define_and_init",
            execute: array_define_and_init,
            enable: true,
        },
        DemoItem {
            title: "array_access",
            execute: array_access,
            enable: true,
        },
        DemoItem {
            title: "update_array",
            execute: update_array,
            enable: true,
        },
        DemoItem {
            title: "for_with_array",
            execute: for_with_array,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

fn array_define_and_init() {
    // 不指定类型和长度，自动推导
    let a = [1, 2, 3, 4, 5]; // a 是一个长度为 5 的整型数组
    let b = [3; 5]; // 等同于 let b = [3, 3, 3, 3, 3]; // 初始值为3，重复5次
    // 指定类型和长度
    let c: [i32; 5] = [1, 2, 3, 4, 5]; // c 是一个长度为 5 的 i32 数组
    let d = [3; 5];
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
    println!("{:?}", [5, 4, 3, 2, 1]);

    // 一个你可能想要使用数组而不是 vector 的例子
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

}

fn array_access() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    /*
    let index = 10;
    let element = a[index]; // index out of bounds: the length is 5 but the index is 10
    println!("The value of element is: {}", element);
     */

}

// 数组访问
fn update_array() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    // a[0] = 123; // 错误：数组 a 不可变

    let mut a = [1, 2, 3];
    a[0] = 4; // 正确
    println!("{:?}", a);
}

fn for_with_array() {
    let a = [1, 2, 3, 4, 5];
    // immutable borrow（不可变借用）
    for x in &a {
        print!("{} ", x);
    }
    println!("");
    let mut a = [1, 2, 3, 4, 5];
    // mutable borrow（可变借用）
    for x in &mut a {
        *x *= 2;
        print!("{} ", x);
    }
    println!("");
/* 这段代码无法通过编译，a.iter()是只读借用
    let mut a = [1, 2, 3, 4, 5];
    for &mut x in a.iter() {
        x *= 2;
        println!("{}",x);
    }

 */
}
