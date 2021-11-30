use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "drop demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "demo_item_drop_trait",
            execute: demo_item_drop_trait,
            enable: true,
        },
        DemoItem {
            title: "demo_item_manul_drop",
            execute: demo_item_manul_drop,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data`{}`!", self.data);
    }
}

fn demo_item_drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
}

fn demo_item_manul_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
    //	c.drop(); //  explicit destructor calls not allowed
    drop(c);
    println!("CustomSmartPointer dropped before the end of function.");
}
