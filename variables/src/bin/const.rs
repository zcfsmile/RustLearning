static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;
static mut COUNTER: i32 = 0;

fn main() {
    println!("This is a program written in {}.", LANGUAGE);
    
    let mut count = 0;
    
    while count < THRESHOLD {
        count += 1;
        println!("Count is: {}", count);
    }

    // 请注意，不安全代码块是存在风险的实践，应仅在必要时使用。
    unsafe {
        COUNTER += 1;
        println!("Counter is now: {}", COUNTER);
    }
}
