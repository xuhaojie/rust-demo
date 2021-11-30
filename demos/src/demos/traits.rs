use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "trait demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "demo_item_impl_trait_for_type",
            execute: demo_item_impl_trait_for_type,
            enable: true,
        },
        DemoItem {
            title: "demo_item_default_trait_method",
            execute: demo_item_default_trait_method,
            enable: true,
        },
        DemoItem {
            title: "demo_item_default_trait_method2",
            execute: demo_item_default_trait_method2,
            enable: true,
        },
        DemoItem {
            title: "demo_item_trait_as_parameter",
            execute: demo_item_trait_as_parameter,
            enable: true,
        },
        DemoItem {
            title: "demo_item_trait_bound",
            execute: demo_item_trait_bound,
            enable: true,
        },
        DemoItem {
            title: "demo_item_bound_multi_trait",
            execute: demo_item_bound_multi_trait,
            enable: true,
        },

        DemoItem {
            title: "demo_item_bound_multi_trait_with_generic",
            execute: demo_item_bound_multi_trait_with_generic,
            enable: true,
        },
        DemoItem {
            title: "demo_item_trait_bound_with_where",
            execute: demo_item_trait_bound_with_where,
            enable: true,
        },
        DemoItem {
            title: "demo_item_largest",
            execute: demo_item_largest,
            enable: true,
        },
        DemoItem {
            title: "demo_item_impl_generic_for_special_trait",
            execute: demo_item_impl_generic_for_special_trait,
            enable: true,
        },
        DemoItem {
            title: "demo_item_blanket_implementation",
            execute: demo_item_blanket_implementation,
            enable: true,
        },
        DemoItem {
            title: "demo_item_operator_override_by_impl_trait2",
            execute: demo_item_operator_override_by_impl_trait2,
            enable: true,
        },
        DemoItem {
            title: "demo_item_same_name_trait_method",
            execute: demo_item_same_name_trait_method,
            enable: true,
        },
        DemoItem {
            title: "demo_item_same_name_trait_method_works",
            execute: demo_item_same_name_trait_method_works,
            enable: true,
        },
        DemoItem {
            title: "demo_item_same_name_associated_functions_and_method",
            execute: demo_item_same_name_associated_functions_and_method,
            enable: true,
        },
        DemoItem {
            title: "demo_item_parent_trait",
            execute: demo_item_parent_trait,
            enable: true,
        },
        DemoItem {
            title: "demo_item_impl_trait_for_external_type",
            execute: demo_item_impl_trait_for_external_type,
            enable: true,
        },

    ];
    execute_demos(&demos);
}

// 为类型实现trai, 在 NewsArticle 和 Tweet 类型上实现 Summary trait
fn demo_item_impl_trait_for_type() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("Hello, world!");
}

// 默认实现, Summary trait 的定义，带有一个 summarize 方法的默认实
fn demo_item_default_trait_method(){
    pub trait Summary{
        // 缺省实现
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // 使用默认实现
    impl Summary for NewsArticle{
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        // 重新定义实现
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

}

fn demo_item_default_trait_method2(){
    pub trait Summary {
        fn summarize_author(&self) -> String;
        // 方法的默认实现
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    // 一旦定义了 summarize_author ，我们就可以对 Tweet 结构体的实例调用 summarize 了，
    // 而summary 的默认实现会调用我们提供的 summarize_author 定义。
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

// trait 作为参数
fn demo_item_trait_as_parameter(){
    pub trait Summary {
        fn summarize_author(&self) -> String;
        // 方法的默认实现
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    // 一旦定义了 summarize_author ，我们就可以对 Tweet 结构体的实例调用 summarize 了，
    // 而summary 的默认实现会调用我们提供的 summarize_author 定义。
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(tweet);
}
// Trait Bound 语法
fn demo_item_trait_bound(){

    pub trait Summary {
        fn summarize_author(&self) -> String;
        // 方法的默认实现
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    // 使用trait bound 语法
    pub fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    // 一旦定义了 summarize_author ，我们就可以对 Tweet 结构体的实例调用 summarize 了，
    // 而summary 的默认实现会调用我们提供的 summarize_author 定义。
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(tweet);
}
// 通过 + 指定多个 trait bound
fn demo_item_bound_multi_trait(){
    use std::fmt::Display;
    pub trait Summary { }
    // 通过 + 指定多个 trait
    pub fn notify(item: impl Summary + Display) {}
}

// + 语法也适用于泛型的 trait bound
fn demo_item_bound_multi_trait_with_generic() {
    use std::fmt::Display;
    pub trait Summary { }
    // + 语法也适用于泛型的 trait bound
    pub fn notify<T: Summary + Display>(item: T) {}
}

// 通过 where 简化 trait bound
fn demo_item_trait_bound_with_where() {
    use std::fmt::Debug;
    use std::fmt::Display;
    fn some_function2<T, U>(t: T, u: U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
    {
        return 0;
    }
}
fn demo_item_return_trait(){
    pub trait Summary {
        fn summarize_author(&self) -> String;
        // 方法的默认实现
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    // 使用trait bound 语法
    pub fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    returns_summarizable();
}

fn demo_item_largest(){
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

// 根据 trait bound 在泛型上有条件的实现方法
fn demo_item_impl_generic_for_special_trait(){
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // 根据 trait bound 在泛型上有条件的实现方法
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

// blanket implementations
// 也可以对任何实现了特定 trait 的类型有条件地实现 trait。
// 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations，他们被广泛的用于 Rust 标准库中。
// 例如，标准库为任何实现了 Display trait 的类型实现了 ToString trait。
fn demo_item_blanket_implementation(){
/*
    use std::fmt::Display;
    impl<T: Display> ToString for T {
        // --snip--
    }
*/
    // 因为标准库有了这些 blanket implementation，我们可以对任何实现了 Display trait 的类型
    // 调用由 ToString 定义的 to_string 方法。例如，可以将整型转换为对应的 String 值，因为整型实现了 Display ：
    let s = 3.to_string();
}

// 关联类型在 trait 定义中指定占位符类型
// 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型。
fn demo_item_associated_types(){
    pub trait Iterator {
        type Item; // Item 是一个占位类型，同时 next 方法定义表明它返回 Option<Self::Item> 类型的值
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    // 区别在于当如示例 19-13 那样使用泛型时，则不得不在每一个实现中标注类型。这是因为我
    // 们也可以实现为 Iterator<String> for Counter ，或任何其他类型，这样就可以有多个
    // Counter 的 Iterator 的实现。换句话说，当 trait 有泛型参数时，可以多次实现这个 trait，
    // 每次需改变泛型参数的具体类型。接着当使用 Counter 的 next 方法时，必须提供类型注解
    // 来表明希望使用 Iterator 的哪一个实现。
    // 通过关联类型，则无需标注类型因为不能多次实现这个 trait。对于示例 19-12 使用关联类型
    // 的定义，我们只能选择一次 Item 会是什么类型，因为只能有一个 impl Iterator for
    // Counter 。当调用 Counter 的 next 时不必每次指定我们需要 u32 值的迭代器。
}

// 默认泛型类型参数和运算符重载
fn demo_item_default_associated_type(){
    /*
     RHS=Self ：这个语法叫做 默认类型参数（default type parameters）。 RHS 是一个
泛型类型参数（“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数。如果实
现 Add trait 时不指定 RHS 的具体类型， RHS 的类型将是默认的 Self 类型，也就是在其
上实现 Add 的类型。
     */
    trait Add<RHS=Self> {
        type Output;
        fn add(self, rhs: RHS) -> Self::Output;
    }
}

fn demo_item_operator_override_by_impl_trait() {
    use std::ops::Add;
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
}

fn demo_item_operator_override_by_impl_trait2() {
    use std::ops::Add;
    struct Millimeters(u32);
    struct Meters(u32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}
// 调用相同名称的方法
fn demo_item_same_name_trait_method(){
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    let person = Human;
    //person.fly(); // multiple applicable items in scope
}

// 完全限定语法与消歧义：调用相同名称的方法, 类似菱形继承问题
fn demo_item_same_name_trait_method_works(){
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person); //  person.fly()
    person.fly();
}

// 完全限定语法与消歧义：调用相同名称的方法, 类似菱形继承问题
fn demo_item_same_name_associated_functions_and_method(){
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name()); //  type annotations required: cannot resolve `_: Animal
    println!("A baby dog is called a {}",  <Dog as Animal>::baby_name());
}

fn demo_item_parent_trait(){
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    impl OutlinePrint for Point {}

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let point = Point{x:12, y:34};
    point.outline_print();
}

// newtype 模式用以在外部类型上实现外部 trait
fn demo_item_impl_trait_for_external_type(){
    use std::fmt;
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        // Display 的实现使用 self.0 来访问其内部的 Vec<T> ，因为 Wrapper 是元组结构体而Vec<T> 是结构体总位于索引 0 的项。接着就可以使用 Wrapper 中 Display 的功能了。
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    /*
此方法的缺点是，因为 Wrapper 是一个新类型，它没有定义于其值之上的方法；必须直接在
Wrapper 上实现 Vec<T> 的所有方法，这样就可以代理到 self.0 上 —— 这就允许我们完全
像 Vec<T> 那样对待 Wrapper 。如果希望新类型拥有其内部类型的每一个方法，为封装类型
实现 Deref trait（第十五章 “通过 Deref trait 将智能指针当作常规引用处理” 部分讨论过）
并返回其内部类型是一种解决方案。如果不希望封装类型拥有所有内部类型的方法 —— 比如
为了限制封装类型的行为 —— 则必须只自行实现所需的方法。
*/
}
