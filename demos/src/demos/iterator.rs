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
            title: "demo_item_iterator_of_vector",
            execute: demo_item_iterator_of_vector,
            enable: true,
        },
        DemoItem {
            title: "demo_item_iterator_of_vector_by_borrow",
            execute: demo_item_iterator_of_vector_by_borrow,
            enable: true,
        },
        DemoItem {
            title: "demo_item_next_method_of_iterator",
            execute: demo_item_next_method_of_iterator,
            enable: true,
        },
        DemoItem {
            title: "demo_item_filter_method_of_iterator",
            execute: demo_item_filter_method_of_iterator,
            enable: true,
        },
        DemoItem {
            title: "demo_item_iterator_with_map",
            execute: demo_item_iterator_with_map,
            enable: true,
        },
        DemoItem {
            title: "demo_item_iterator_with_collect",
            execute: demo_item_iterator_with_collect,
            enable: true,
        },
        DemoItem {
            title: "demo_item_iterator_with_rev",
            execute: demo_item_iterator_with_rev,
            enable: true,
        },
        DemoItem {
            title: "demo_item_iterator_with_max",
            execute: demo_item_iterator_with_max,
            enable: true,
        },
        DemoItem {
            title: "demo_item_iterator_with_sum",
            execute: demo_item_iterator_with_sum,
            enable: true,
        },
        DemoItem {
            title: "demo_item_iterator_with_fold",
            execute: demo_item_iterator_with_fold,
            enable: true,
        },
        DemoItem {
            title: "demo_item_iterator_with_scan",
            execute: demo_item_iterator_with_scan,
            enable: true,
        },
    ];
    execute_demos(&demos);
}
/*
Rust语言中的迭代器是实现了Iterator trait的类型，并需要至少实现一个next函数，用于让迭代器指向下一个迭代对象，并返回一个Option<T>用于指示对象是否存在。
next函数定义大致如下，Item为一个关联类型，表示所迭代的对象的类型。
fn next(&mut self) -> Option<Self::Item>;
*/

fn demo_item_iterator_of_vector() {
    let v = vec![1, 2, 3, 4, 5];
    for x in v.iter() {
        println!("x={}", x);
    }

    // or
    for x in &v {
        println!("x={}", x);
    }
}

fn demo_item_iterator_of_vector_by_borrow() {
    let v = vec![1, 2, 3, 4, 5];
    for x in &v {
        println!("x={}", x);
    }
    /*
        //如果我们想要使用可变的迭代器将元素添加到向量中，如下所示：

        let mut nums = vec![1, 2, 3, 4, 5];
        for i in &mut nums {
            nums.push(*i);
        }
        println!("{:?}", nums);
        它不编译，并抛出错误信息cannot borrow `nums` as mutable more than once at a time。
        你看，我们的迭代器（在for循环中实例化）已经借用nums作为可变。 push表达试图再次这样做，这是禁止的。 这是在Rust中著名的安全机制。
        如果我们可以将某个push入向量中，同时迭代它，则会导致迭代器失效，从而导致未定义的行为。
        Rust可以在编译时防止发生这种情况。 迭代器不仅强大，而且它们也是超级安全的。
    */
}

fn demo_item_next_method_of_iterator() {
    let a = [1, 2, 3];

    let mut iter = a.iter();
    // A call to next() returns the next value...
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());

    // ... and then None once it's over.
    assert_eq!(None, iter.next());

    // More calls may or may not return `None`. Here, they always will.
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
}

/*
filter
filter是在Iterator trait内默认实现的一个函数，只要用户自定义的类型实现了Iterator trait，那么filter就会自动提供给用户。
它的作用就如名字一样，过滤掉迭代过程中不满足某个条件的元素，它的参数是一个闭包，其返回值为bool类型，指示该元素是否符合条件。
*/
fn demo_item_filter_method_of_iterator() {
    for x in (0..=10).filter(|i| i % 3 == 0) {
        println!("x={} ", x);
    }
}

/*
map
顾名思义,map即是对迭代的元素进行一次映射后再返回映射后的结果。
比如我要把一个i32数组的每个元素都乘以2，并且迭代访问每个结果，那么就可以这么写。
原理也是通过包装原迭代器，读者可以自己仿照上面的filter实现方式实现一下map。
*/
fn demo_item_iterator_with_map() {
    let vec = vec![1, 2, 3, 4, 5];
    for num_str in vec.iter().map(|x| x * 2) {
        println!("{}", num_str);
    }
}

/*
collect
collect是将一个迭代器迭代的所有元素组合成一个新的集合，比如我要生成一个存有0到100的Vec<i32>，就可以这么写。
let vec = (0..=100).collect::<Vec<_>>();//Vec的泛型参数可以不写，由编译器推导为i32.
*/
fn demo_item_iterator_with_collect() {
    let vec = vec![1, 2, 3, 4, 5];
    let str_vec = vec.iter().map(|x| x.to_string()).collect::<Vec<_>>(); //这里的str_vec就是一个Vec<String>了
}

fn demo_item_iterator_with_rev() {
    for i in (0..=10).rev() {
        println!("{} ", i);
    }
}

/*
max
max是求迭代元素的最大值，比较简单不多说，给个例子。
*/
fn demo_item_iterator_with_max() {
    let vec = vec![1, 5, 3, 4, 2];
    let max = vec.iter().max().unwrap();
    println!("{}", max); //输出5
}

/*
sum
sum是求迭代元素的和，需要指定一下结果的类型。
*/
fn demo_item_iterator_with_sum() {
    let vec = vec![1, 2, 3, 4, 5];
    let sum = vec.iter().sum::<i32>();
    println!("{}", sum); //输出15
}

/*
fold
fold是一个神奇的函数，它有两个参数，第一个是初始值，第二个是一个闭包，闭包第一个参数是一个累计值，第二个参数是本次迭代元素的引用，返回值作为下一次迭代的累计值。
接触过其他函数式语言的读者可能对这个函数非常熟悉。
这么说可能难以理解，举个例子，还是求和，C中这么写。
int sum=0;
int a[5]={1,2,3,4,5};
for(int i=0;i<100;++i){
    sum+=array[i];
}
*/
fn demo_item_iterator_with_fold() {
    let vec = vec![1, 2, 3, 4, 5];
    let res = vec.iter().fold(0, |acc, x| acc + x);
    println!("{}", res);
}

/*
scan
scan和fold很类似，但是它允许你直接修改累计值，并且允许你选择什么时候停止迭代，取决于你传入的闭包何时返回None。
比如我不仅要求数组的和，还要获取每次累加的结果，就可以这么写。
*/
fn demo_item_iterator_with_scan() {
    let vec = vec![1, 2, 3, 4, 5];
    for step in vec.iter().scan(0, |acc, x| {
        *acc += *x;
        Some(*acc)
    }) {
        println!("{} ", step);
    } //打印1 3 6 10 15
}
