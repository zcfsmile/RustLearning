#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
// #[alloc()]
fn main() {
    // 常用表达式示例
    // 一些字面量：可以直接表达一个特定的值
    let integer = 42;
    let character = 'a';
    let boolean = true;
    let unit = ();

    // 常量声明
    const MAX_POINTS: u32 = 100_000;

    // 变量绑定
    let x = 10;
    let mut y = 20;
    y += 5;

    // 函数表达式
    print_name("Rust"); 

    let m = 1;
    let n = add_one(m);

    // 块表达式
    let y = {
        let x = 1;
        x + 1
    };

    // if 
    let number = 7;
    let result = if number < 5 {
        "less"
    } else if number < 10 {
        "more"
    } else {
        ""
    };

    // 循环
    // loop 循环
    let mut num = 10;
    // loop 可以返回值
    let result = loop {
        num -= 1;
        if num == 0 {
            break "num == 0";
        }
    };
    // 多个loop循环嵌套，可添加标签
    let mut count = 0;
    'outer: loop {
        println!("外层循环: {}", count);
        count += 1;
        let mut inner_count = 0;
        'inner: loop {
            println!("内层循环: {}", inner_count);
            inner_count += 1;
            if inner_count == 3 {
                // break 'inner;
                continue 'outer;
            }
            if count == 2 {
                break 'outer;
            }
        }
    }

    for elem in [10, 20, 30].iter() {
        println!("{}", elem);
    }
}

fn print_name(name: &str) {
    println!("hello {}", name);
}

fn add_one(n: i32) -> i32 {
    n + 1
}

