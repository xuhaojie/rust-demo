use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "generic demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "demo_item_generic_function",
            execute: demo_item_generic_function,
            enable: true,
        },
        DemoItem {
            title: "demo_item_generic_with_struct",
            execute: demo_item_generic_with_struct,
            enable: true,
        },
        DemoItem {
            title: "demo_item_generic_struct_with_multi_type",
            execute: demo_item_generic_struct_with_multi_type,
            enable: true,
        },
        DemoItem {
            title: "demo_item_generic_with_trait",
            execute: demo_item_generic_with_trait,
            enable: true,
        },

        DemoItem {
            title: "demo_item_generic_with_struct_method",
            execute: demo_item_generic_with_struct_method,
            enable: true,
        },
        DemoItem {
            title: "demo_item_generic_with_struct_method_for_special_type",
            execute: demo_item_generic_with_struct_method_for_special_type,
            enable: true,
        },
        DemoItem {
            title: "demo_item_generic_with_enum",
            execute: demo_item_generic_with_enum,
            enable: true,
        },

    ];
    execute_demos(&demos);
}

// 一个可以用于任何实现了 PartialOrd 和 Copy trait 的泛型的 largest 函数
fn demo_item_generic_function(){
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// 在 Point<T> 结构体上实现方法 x ，它返回 T 类型的字段 x 的引用
fn demo_item_generic_with_struct() {
    struct Point<T> {
        x: T,
        y: T,
    }
    let p = Point { x: 5, y: 10 };
    println!("interger = Point({}, {})", p.x, p.y);
    let p = Point { x: 1.1, y: 4.2 };
    println!("float = Point({}, {})", p.x, p.y);
}

// 在 Point<T> 结构体上实现方法 x ，它返回 T 类型的字段 x 的引用
fn demo_item_generic_struct_with_multi_type() {
    struct Point<T, U> {
        x: T,
        y: U,
    }
    let both_integer = Point { x: 5, y: 10 };
    println!("both_integer = Point({}, {})", both_integer.x, both_integer.y);
    let both_float = Point { x: 1.1, y: 4.2 };
    println!("both_float = Point({}, {})", both_float.x, both_float.y);
    let integer_and_float = Point { x: 5, y: 4.1 };
    println!("integer_and_float = Point({}, {})", integer_and_float.x, integer_and_float.y);
}

fn demo_item_generic_with_trait() {
    struct Point<T, U> {
        x: T,
        y: U,
    }
    fn display_point<T,U>(p : &Point<T,U>)
        where T: std::fmt::Debug, U: std::fmt::Debug  {
        println!("pint = Point({:?}, {:?})", p.x, p.y);
    }
    let both_integer = Point { x: 5, y: 10 };
    display_point::<_,_>(&both_integer);

    let both_float = Point { x: 1.1, y: 4.2 };
    display_point(&both_float);

    let integer_and_float = Point { x: 5, y: 4.1 };
    display_point(&integer_and_float);

}

// 在 Point<T> 结构体上实现方法 x ，它返回 T 类型的字段 x 的引用
fn demo_item_generic_with_struct_method() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// 构建一个只用于拥有泛型参数 T 的结构体的具体类型的 impl 块
fn demo_item_generic_with_struct_method_for_special_type() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let p = Point { x: 3.0, y: 4.0 };
    println!("distance form origin = {}", p.distance_from_origin());
}


// 在 Point<T> 结构体上实现方法 x ，它返回 T 类型的字段 x 的引用
fn demo_item_generic_with_enum() {
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

}

// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率
fn demo_item_monomorphization(){
    enum MyOption<T> {
        Some(T),
        None,
    };

    let integer= MyOption::<i32>::None; // 由于None不能确定具体实例化的数据类型，必须手工给出
    let integer= MyOption::Some(5);     // 如果不嫌麻烦也可以这么写 MyOption::<i32>::Some(5);

    // 针对i32类型的实现，编译器会生成类似下面的代码
    enum MyOption_i32 {
        Some(i32),
        None,
    };

    let integer= MyOption_i32::None;
    let integer = MyOption_i32::Some(5);


    let float= MyOption::<f64>::None;
    let float = MyOption::Some(5.0);

    // 针对f64类型的实现，编译器会生成类似下面的代码
    enum MyOption_f64 {
        Some(f64),
        None,
    };
    let float = MyOption_f64::None;
    let float = MyOption_f64::Some(5.0);
}
