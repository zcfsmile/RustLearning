use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    // 插入
    heap.push(10);
    // 弹出
    let v = heap.pop();
    println!("v: {}", v.unwrap()); // v: 10
    heap.push(20);
    heap.push(5);
    // 返回堆中最大值的引用
    println!("max: {:?}", heap.peek()); // max: Some(20)
}
