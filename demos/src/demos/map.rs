use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "map demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "map_basic_usage",
            execute: map_basic_usage,
            enable: true,
        },
        DemoItem {
            title: "map_from_vectors",
            execute: map_from_vectors,
            enable: true,
        },
        DemoItem {
            title: "access_map",
            execute: access_map,
            enable: true,
        },
        DemoItem {
            title: "map_insert",
            execute: map_insert,
            enable: true,
        },
        DemoItem {
            title: "map_insert_when_not_exists",
            execute: map_insert_when_not_exists,
            enable: true,
        },
        DemoItem {
            title: "map_update",
            execute: map_update,
            enable: true,
        },
        DemoItem {
            title: "map_and_ownership",
            execute: map_and_ownership,
            enable: true,
        },
        DemoItem {
            title: "elements_dropped_with_map",
            execute: elements_dropped_with_map,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

use std::collections::HashMap;
fn map_basic_usage() {
    // let mut scores = HashMap::new();
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores={:?}", scores); // value borrowed here after move
}

fn map_from_vectors() {
    let teams = vec![String::from("Blue"), String::from("Yello")];
    let initial_scores = vec![10, 50];
    // 这里 HashMap<_, _> 类型注解是必要的，因为可能 collect 很多不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。
    // 但是对于键和值的类型参数来说，可以使用下划线	占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let scores: HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores={:?}", scores); // value borrowed here after move
}

fn access_map() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 访问哈希 map 中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score={:?}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn map_insert(){
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
// 覆盖一个值
fn map_overwrite() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
}

fn map_insert_when_not_exists() {
    // 只在键没有对应值时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // entry 方法只在键没有对应一个值时插入
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

// 根据旧值更新一个值
fn map_update() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map)
}

fn map_and_ownership() {
    // 哈希 map 和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("field_name={},field_value={}", field_name, field_value); // value borrowed here after move
}

fn elements_dropped_with_map() {
    use std::collections::HashMap;
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    {
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
    }// field_name,field_value dorpped with map;
//    println!("{} {}",field_name,field_value); // borrow of moved value: `field_name`
}