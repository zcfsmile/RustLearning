use std::{sync::Arc, thread};

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    // 创建多个线程，每个线程共享数据
    let mut handles = vec![];

    for i in 0..5 {
        // Arc 引用计数增加
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {} sees data as: {:?}", i, data_clone);
        });
        handles.push(handle);
    }

    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出 `Arc` 的引用计数（可选）
    println!("Reference count: {}", Arc::strong_count(&data));
    println!("Data: {:?}", data);
}
