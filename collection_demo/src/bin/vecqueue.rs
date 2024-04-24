#![allow(unused_variables)]
use std::collections::VecDeque;

fn main () {
    let mut queue: VecDeque<i32> = VecDeque::new();
    // 首部添加
    queue.push_front(1);
    // 尾部添加
    queue.push_back(2);
    // 队首弹出
    let v1 = queue.pop_front();
    // 队尾弹出
    let v2 = queue.pop_back();
}