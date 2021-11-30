use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "vector demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
     DemoItem {
         title: "demo_item_vector_init",
         execute: demo_item_vector_init,
         enable: true,
     },
     DemoItem {
         title: "demo_item_vector_with_marco",
         execute: demo_item_vector_with_marco,
         enable: true,
     },
     DemoItem {
         title: "demo_item_vector_elements_dropped_with_vector",
         execute: demo_item_vector_elements_dropped_with_vector,
         enable: true,
     },
     DemoItem {
         title: "demo_item_vector_access_with_get",
         execute: demo_item_vector_access_with_get,
         enable: true,
     },
     DemoItem {
         title: "demo_item_vector_access_with_operator",
         execute: demo_item_vector_access_with_operator,
         enable: true,
     },
     DemoItem {
         title: "demo_item_vector_push_while_access",
         execute: demo_item_vector_push_while_access,
         enable: true,
     },
     DemoItem {
         title: "demo_item_vector_iterate_with_for",
         execute: demo_item_vector_iterate_with_for,
         enable: true,
     },
     DemoItem {
         title: "demo_item_vector_update_with_iteration",
         execute: demo_item_vector_update_with_iteration,
         enable: true,
     },
     DemoItem {
         title: "demo_item_vector_store_multi_types_with_enum",
         execute: demo_item_vector_store_multi_types_with_enum,
         enable: true,
     },

    ];
    execute_demos(&demos);
}

fn demo_item_vector_init(){
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v={:?}", &v);
    println!("v[1]={}", &v[1]);
}

fn demo_item_vector_with_marco(){
    let v = vec![4, 5, 6];
    println!("v={:?}", v);
}

fn demo_item_vector_elements_dropped_with_vector(){
    let s1 = String::from("1");
    let s2 = String::from("2");
    {
        let v = vec![s1];
        // 处理变量 v
    } // <- 这里 v 离开作用域并被丢弃
    //println!("s1 = {}", s1); // -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    println!("s2 = {}", s2);
}
fn demo_item_vector_access_with_get(){
    let v = vec![4, 5, 6];
    println!("v={:?}", v);
    match v.get(3) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    let does_not_exist = v.get(100);
    println!("does_not_exist={:?}", does_not_exist);
}

fn demo_item_vector_access_with_operator(){
    let v = vec![4, 5, 6];
    let first = &v[0];
    println!("first={:?}", first);
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    // let does_not_exist = v[100]; // 这会导致panic
    // println!("does_not_exist={:?}", does_not_exist);
}

fn demo_item_vector_push_while_access(){
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("The first element is: {}", first); // this ok
    v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable
    // but this will failed  不能在相同作用域中同时存在可变和不可变引用的规则
//    println!("The first element is: {}", first); // cannot borrow `v` as mutable because it is also borrowed as immutable

//   v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable
//   ^^^^^^^^^ mutable borrow occurs here
//   println!("The first element is: {}", first);
//                                          ----- immutable borrow later used here
}
// 通过 for 循环遍历 vector 的元素并打印
fn demo_item_vector_iterate_with_for() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

// 以遍历可变 vector 的每一个元素的可变引用以便能改变他们
fn demo_item_vector_update_with_iteration() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}

// 使用枚举来储存多种类型
fn demo_item_vector_store_multi_types_with_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for r in &row {
        println!("{:?}", r);
    }
}
