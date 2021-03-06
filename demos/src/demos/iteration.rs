use crate::execute_demos;
use crate::DemoItem;

pub fn new() -> DemoItem {
    DemoItem {
        title: "function demos",
        execute: execute,
        enable: true,
    }
}

fn execute() {
    let demos = vec![
        DemoItem {
            title: "demo_item_for",
            execute: demo_item_for,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_custom_step",
            execute: demo_item_for_with_custom_step,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_end",
            execute: demo_item_for_with_end,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_var",
            execute: demo_item_for_with_var,
            enable: true,
        },
        DemoItem {
            title: "demo_item_reverse_for",
            execute: demo_item_reverse_for,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_chain",
            execute: demo_item_for_with_chain,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_iterator",
            execute: demo_item_for_with_iterator,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_reverse_iterator",
            execute: demo_item_for_with_reverse_iterator,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_string",
            execute: demo_item_for_with_string,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_string_reverse",
            execute: demo_item_for_with_string_reverse,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_vector",
            execute: demo_item_for_with_vector,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_filter",
            execute: demo_item_for_with_filter,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_with_enumerate",
            execute: demo_item_for_with_enumerate,
            enable: true,
        },
        DemoItem {
            title: "demo_item_complex_for",
            execute: demo_item_complex_for,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_and_complex_iterator",
            execute: demo_item_for_and_complex_iterator,
            enable: true,
        },
        DemoItem {
            title: "demo_item_for_and_zip",
            execute: demo_item_for_and_zip,
            enable: true,
        },
        DemoItem {
            title: "demo_item_while",
            execute: demo_item_while,
            enable: true,
        },
        DemoItem {
            title: "demo_item_loop",
            execute: demo_item_loop,
            enable: true,
        },
        DemoItem {
            title: "demo_item_the_truth_of_for",
            execute: demo_item_the_truth_of_for,
            enable: true,
        },
        DemoItem {
            title: "demo_item_loop_break_with_lable",
            execute: demo_item_loop_break_with_lable,
            enable: true,
        },
        DemoItem {
            title: "demo_item_loop_with_break_value",
            execute: demo_item_loop_with_break_value,
            enable: true,
        },
    ];
    execute_demos(&demos);
}

fn demo_item_for() {
    for x in 1..4 {
        println!("x={}", x);
    }
    // ???????????????????????????
    for x in 4..1 {
        println!("x={}", x);
    }
}

fn demo_item_for_with_var() {
    let m = 2;
    let n = 10;
    for x in m..n {
        println!("x={}", x);
    }
}
fn demo_item_for_with_end() {
    for x in 1..=4 {
        println!("x={}", x);
    }
}

fn demo_item_reverse_for() {
    for x in (1..4).rev() {
        println!("x={}", x);
    }
}

fn demo_item_for_with_custom_step() {
    for x in (1..10).step_by(2) {
        println!("{}", x);
    }
}

fn demo_item_for_with_chain() {
    let c = (1..4).chain(6..9);
    for i in c {
        print!("{} ", i);
    }
    // output: 1 2 3 6 7 8
}

fn demo_item_for_with_iterator() {
    let v = vec![1, 2, 3, 4, 5];
    for x in v.iter() {
        println!("x={}", x);
    }
}

fn demo_item_for_with_reverse_iterator() {
    let v = vec![1, 2, 3, 4, 5];
    for x in v.iter().rev() {
        println!("x={}", x);
    }
}

fn demo_item_for_with_string() {
    let s = String::from("hello??????");
    for c in s.chars() {
        println!("{}", c);
    }
}

fn demo_item_for_with_string_reverse() {
    let s = String::from("hello??????");
    for c in s.chars().rev() {
        println!("{}", c);
    }
}

fn demo_item_for_with_vector() {
    let v = vec![1, 2, 3, 4, 5];
    for x in v.iter() {
        println!("{}", x);
    }
}

fn demo_item_for_with_filter() {
    for x in (0..21).filter(|i| (i % 2 == 0)) {
        println!("{}", x);
    }
}

fn demo_item_for_with_enumerate() {
    let vec = vec![1, 2, 4, 8, 16, 32];
    for (i, x) in vec.iter().enumerate() {
        println!("2^{}={}", i, x);
    }
}

fn demo_item_complex_for() {
    let m = 2;
    let n = 10;
    for x in (m..n).step_by(3).filter(|i| (i % 2 == 0)).rev() {
        println!("x={}", x);
    }

    let v = (m..n).step_by(3).filter(|i| (i % 2 == 0)).rev();
    for x in v {
        println!("x={}", x);
    }
}

fn demo_item_for_and_complex_iterator() {
    let r = (1..20).filter(|&x| x % 5 == 0).chain((6..9).rev());

    for i in r {
        print!("{} ", i);
    }
    // output: 5 10 15 8 7 6
}

fn demo_item_for_and_zip() {
    let cities = ["Toronto", "New York", "Melbourne"];
    let populations = [2_615_060, 8_550_405, 4_529_500];
    let matrix = cities.iter().zip(populations.iter());
    for (c, p) in matrix {
        println!("{:10}: population = {}", c, p);
    }
    // output:
    // Toronto   : population = 2615060
    // New York  : population = 8550405
    // Melbourne : population = 4529500
}

fn demo_item_while() {
    let mut n = 0;
    while n < 4 {
        println!("n={}", n);
        n += 1; // no n++
    }
}

fn demo_item_loop() {
    let mut n = 0;
    loop {
        println!("n={}", n);
        n += 1; // no n++
        if n > 3 {
            break;
        }
    }
}
/*
Rust???for????????????????????????????????????in??????????????????????????????????????????for??????????????????????????????????????????next??????in???????????????????????????????????????????????????????????????next??????Option::None??????????????????
?????????????????????????????????????????????????????????????????????????????????for?????????????????????
???????????????????????????????????????????????????????????????while???????????????loop????????????
*/
fn demo_item_the_truth_of_for() {
    let v = vec![1, 2, 3, 4, 5];
    for x in v.iter() {
        println!("x={}", x);
    }
    // ?????????
    let mut iter = v.iter();
    loop {
        match iter.next() {
            None => break,
            Some(x) => {
                println!("x={}", x);
            }
        }
    }
}

/*
???????????????????????????????????? break ??? continue ???????????????
????????????????????????????????? ????????? 'label??????????????????????????????????????????????????? break/continue ?????????
*/
fn demo_item_loop_break_with_lable() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // ??????????????????????????????
            //break;

            // ????????????????????????
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

/*
loop ??????????????????????????????????????????????????????
?????????????????????????????????????????????????????? ???????????????????????????
??????????????? break ????????????????????? loop ??????????????????
*/
fn demo_item_loop_with_break_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
