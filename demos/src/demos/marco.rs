use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "marcos demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "demo_marco_basic",
            execute: demo_marco_basic,
            enable: true,
        },
        DemoItem {
            title: "demo_marco_ident",
            execute: demo_marco_ident,
            enable: true,
        },
        DemoItem {
            title: "demo_marco_overload",
            execute: demo_marco_overload,
            enable: true,
        },
        DemoItem {
            title: "demo_marco_repeat",
            execute: demo_marco_repeat,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

// 这是一个简单的宏，名为 `say_hello`。
macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数。
    () => {
        // 此宏将会展开成这个代码块里面的内容。
        println!("Hello!");
    };
}

fn demo_marco_basic() {
    say_hello!()
}

// 指示符, 这里列出全部指示符：
// block
// expr 用于表达式
// ident 用于变量名或函数名
// item
// pat (模式 pattern)
// path
// stmt (语句 statement)
// tt (标记树 token tree)
// ty (类型 type)

macro_rules! create_function {
    // 此宏接受一个 `ident` 指示符表示的参数，并创建一个名为 `$func_name` 的函数。
    // `ident` 指示符用于变量名或函数名
    ($func_name:ident) => {
        fn $func_name() {
            // `stringify!` 宏把 `ident` 转换成字符串。
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}
// 借助上述宏来创建名为 `foo` 和 `bar` 的函数。
create_function!(foo);
create_function!(bar);

// `ident` 指示符用于变量名或函数名
fn demo_marco_ident() {
    foo();
    bar();
}

// expr

macro_rules! print_result {
    // 此宏接受一个 `expr` 类型的表达式，并将它作为字符串，连同其结果一起
    // 打印出来。
    // `expr` 指示符表示表达式。
    ($expression:expr) => {
        // `stringify!` 把表达式*原样*转换成一个字符串。
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

fn demo_marco_expr() {
    print_result!(1u32 + 1);

    // 回想一下，代码块也是表达式！
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}

// 重载
// 宏可以重载，从而接受不同的参数组合。
// 在这方面，macro_rules! 的作用类似于 匹配（match）代码块：
macro_rules! test {
    // 参数不需要使用逗号隔开。
    // 参数可以任意组合！
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // ^ 每个分支都必须以分号结束。
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

fn demo_marco_overload() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}

// 重复
// 宏在参数列表中可以使用 + 来表示一个参数可能出现一次或多次，使用 * 来表示该 参数可能出现零次或多次。
// 在下面例子中，把模式这样： $(...),+ 包围起来，就可以匹配一个或多个用逗号隔开 的表达式。
// 另外注意到，宏定义的最后一个分支可以不用分号作为结束。
// `min!` 将求出任意数量的参数的最小值。
macro_rules! find_min {
    // 基本情形：
    ($x:expr) => ($x);
    // `$x` 后面跟着至少一个 `$y,`
    ($x:expr, $($y:expr),+) => (
        // 对 `$x` 后面的 `$y` 们调用 `find_min!`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn demo_marco_repeat() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}

// DRY (不写重复代码)
// 通过提取函数或测试集的公共部分，宏可以让你写出 DRY 的代码（DRY 是 Don't Repeat Yourself 的缩写，意思为 “不要写重复代码”）。
// 这里给出一个例子，对 Vec<T> 实现 并测试了关于 +=、*= 和 -= 等运算符。

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // `tt`（token tree，标记树）指示符表示运算符和标记。
    ($a:ident, $b: ident, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

// 实现 `add_assign`、`mul_assign` 和 `sub_assign` 等函数。
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func: ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            pub fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        };
    }

    // 测试 `add_assign`、`mul_assign` 和 `sub_assign`
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);

    fn demo_macro_test() {
        test!(add_assign, 1u32, 2u32, 3u32);
        test!(mul_assign, 2u32, 3u32, 6u32);
        test!(sub_assign, 3u32, 2u32, 1u32);
    }
}

// DSL（领域专用语言）
// DSL 是 Rust 的宏中集成的微型 “语言”。
// 这种语言是完全合法的，因为宏系统会把它转换 成普通的 Rust 语法树，它只不过看起来像是另一种语言而已。
// 这就允许你为一些特定功能 创造一套简洁直观的语法（当然是有限制的）。

macro_rules! calculate {
    // 单个 `eval` 的模式
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // 递归地拆解多重的 `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn demo_macro_calculate() {
    calculate! { // 妈妈快看，可变参数的 `calculate!`！
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
