use std::collections::HashSet;

fn main() {
    let mut set1 = HashSet::new();
    // 插入
    set1.insert(10);
    set1.insert(20);
    set1.insert(10);
    println!("set1: {:?}", set1); // set1: {20, 10}
    // 是否包含
    println!("{}", set1.contains(&10)); // true
    // 是否为空
    println!("{}", set1.is_empty()); // false
    // 长度
    println!("{}", set1.len()); // 2
    // 删除
    set1.remove(&10);
    println!("set1: {:?}", set1); // set1: {20}

    set1.insert(5);
    set1.insert(15);
    set1.insert(25);
    // 保留符合要求的
    set1.retain(|s| *s > 20);
    println!("set1: {:?}", set1); // set1: {25}

    // 迭代
    // 共享引用
    for m in &set1 {
        println!("m: {}", m);
    }
    // 按值迭代
    for v in set1 {
        println!("v: {}", v);
    }

    // 对整个 set 的操作
    let set2 = HashSet::from([4, 6, 7]);
    let set3 = HashSet::from([3, 4, 5]);
    
    // 交集
    for v in set2.intersection(&set3) {
        println!("{}", v); // 4
    }

    // 并集
    for v in set2.union(&set3) {
        println!("{}", v);
    }

    // 差集
    for v in set2.difference(&set3) {
        println!("{}", v);
    }

}