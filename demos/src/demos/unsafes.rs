use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "unsafe demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![DemoItem {
        title: "unsafe",
        execute: run,
        enable: true,
    }];
    execute_demos(&demos);
}

unsafe fn dangerous() {}

/* *slice` as mutable more than once at a time
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid],
     &mut slice[mid..])
}
*/

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}
// 使用 extern 函数调用外部代码
extern "C" {
    fn abs(input: i32) -> i32;
}

// 从其它语言调用 Rust 函数
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// 全局变量在 Rust 中被称为 静态（static）变量
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 实现不安全 trait
unsafe trait Foo {
    fn bar();
}

unsafe impl Foo for i32 {
    fn bar() {}
}
fn run() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 创建一个指向任意内存地址的裸指针
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is:{}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
