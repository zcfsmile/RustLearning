use std::thread;

fn main() {
    let task1 = || {
        println!("Task 1 is running");
        42
    };
    let task2 = || {
        println!("Task 2 is running");
        58
    };

    // Fork: 启动两个任务
    let handle1 = thread::spawn(task1);
    let handle2 = thread::spawn(task2);

    // Join: 等待两个任务结束
    let result1 = handle1.join().expect("Thread 1 panicked");
    let result2 = handle2.join().expect("Thread 2 panicked");

    // 汇总结果
    println!("Result: {} + {} = {}", result1, result2, result1 + result2);
}
