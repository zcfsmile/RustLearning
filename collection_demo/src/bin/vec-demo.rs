#![allow(unused_variables)]
#![allow(unused_mut)]
use rand::seq::SliceRandom;
use rand::thread_rng;
fn main() {
    // 创建向量
    let mut numbers: Vec<i32> = vec![];
    let mut strings: Vec<String> = Vec::with_capacity(10);
    let words = vec!["step", "on", "no", "pets"];
    let str_nums = vec!["1", "1", "2", "3", "4"];
    let string_nums = str_nums.into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    // 通过索引来获取数组、切片、向量的元素
    // 获取某个元素的引用
    let first_string = &string_nums[0];
    // 获取某个元素的副本
    let mut nums = vec![1, 2, 3, 4, 5];
    let two_num = nums[1]; // 要求实现 Copy trait
    let twr_string = string_nums[1].clone(); // 要求实现了 Clone trait
    // 获取切片的引用
    let nums_ref = &nums[0..3];
    // 获取切片的副本，转换一个向量
    let nums_copy = nums[0..3].to_vec(); // 要求实现 Clone trait
    // 获取第一个元素, Option 类型
    let one_num = nums.first();
    // 获取最后一个元素, Option 类型
    let last_num = nums.last();
    // 通过下标获取, Option 类型
    let two_num = nums.get(1);
    // 获取可变的元素
    let mut mut_one = nums.first_mut();
    let mut mut_last = nums.last_mut();
    let mut two_mut = nums.get_mut(1);

    // 迭代：使用 iter 和 iter_mut 返回迭代器
    let mut nums_iter = nums.iter();
    println!("{:?}", nums_iter.next()); // Some(1)

    // len 返回向量的长度
    println!("{}", nums.len()); // 5

    // is_empty 判断是否为空
    println!("{}", nums.is_empty()); // false

    // capacity 获取容量
    println!("{}", strings.capacity()); // 10

    // reserve(n) 确保向量至少有足够的备用容量来容纳另外 n 个元素” ,调用之后 vec.capacity() 至少等于 vec.len() + n”
    strings.reserve(20);
    println!("{}", strings.capacity()); // 20
    // 添加元素
    strings.push("a".to_string());
    // reserve_exact(n) 精确指定，调用之后 vec.capacity() 等于 vec.len() + n
    strings.reserve_exact(20);
    println!("{}", strings.capacity()); // 21
    // 移除并返回最后一个元素
    let last_v = strings.pop();
    println!("{:?}", last_v); // Some("a")

    // insert(index, vaue) 插入
    strings.insert(0, "a".to_string());
    strings.insert(1, "b".to_string());
    strings.insert(2, "c".to_string());

    // remove(index) 移除
    strings.remove(0);

    // 更改向量的长度
    // 设置长度，并填充新空间
    strings.resize(10, "a".to_string());
    println!("{:?}", strings); // ["b", "c", "a", "a", "a", "a", "a", "a", "a", "a"]
    // 设置长度，使用闭包来填充新空间
    strings.resize_with(20, || "d".to_string());
    println!("{:?}", strings); // ["b", "c", "a", "a", "a", "a", "a", "a", "a", "a", "d", "d", "d", "d", "d", "d", "d", "d", "d", "d"]
    // 截断: 减少到指定的长度
    strings.truncate(5);
    println!("{:?}", strings); // ["b", "c", "a", "a", "a"]
    // 清空
    strings.clear();
    println!("{:?}", strings); // []
    println!("{}", strings.capacity()); // 21

    // 一次添加或移除多个值
    // 在末尾添加来自给定迭代器的所有条目, push 的多版本
    let to_add_strings = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    strings.extend(to_add_strings.into_iter()); 
    println!("{:?}", strings);// ["a", "b", "c"]
    // 从末尾移除多个值，pop 的多版本
    let split_stirng = strings.split_off(2);
    println!("{:?}", split_stirng); // ["c"]
    println!("{:?}", strings); // ["a", "b"]
    // append(&mut vec2) 将 vec2 的元素添加到向量，vec2 会被清空
    let mut vec2 = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    strings.append(&mut vec2);
    println!("{:?}", vec2); // []
    println!("{:?}", strings); // ["a", "b", "1", "2", "3"]

    // drain(range) - 抽取，移除 range 范围内的切片
    let range_strings: Vec<String> = strings.drain(1..3).collect();
    println!("{:?}", range_strings); // ["b", "1"]
    println!("{:?}", strings); // ["a", "2", "3"]

    // retain(test) - 留下满足 test 闭包的元素
    strings.retain(|s| s.contains('a'));
    println!("{:?}", strings); // ["a"]

    // dedup() - 去重 
    // dedup_by(same) - 根据 same 闭包调用结果去重复
    // dedup_by_key(key) - 根据 key 属性去重
    strings.push("a".to_string());
    strings.dedup();
    println!("{:?}", strings); // ["a"]

    // 联结
    // slices.concat()
    let slices = [[1, 2], [3, 4], [5, 6]].concat();
    println!("slices: {:?}", slices); // slices: [1, 2, 3, 4, 5, 6]
    // slices.join(&separater)
    let slices2 = [[1, 2], [3, 4], [5, 6]].join(&0);
    println!("slices2: {:?}", slices2); // slices2: [1, 2, 0, 3, 4, 0, 5, 6]

    // 拆分
    let mut vec1 = vec![11, 2, 33, 0, 44, 66, 0];
    // slice.iter() 或 slice.iter_mut() 生成对 slice 中每个元素的引用
    println!("iter: {:?}", vec1.iter().next()); // iter: Some(11)
    // slice.split_at(index) slice.split_at_mut(index) 
    let (left, right) = vec1.split_at(3);
    println!("left: {:?}", left); // [11, 2, 33]
    println!("right: {:?}", right); // [0, 44, 66, 0]
    // 拆分首个 split_first
    if let Some((first, elements)) = vec1.split_first() {
        println!("first: {:?}", first); // first: 11
        println!("elements: {:?}", elements); // elements: [2, 33, 0, 44, 66, 0]
    }
    // 拆分末尾 split_last()
    // split(is_sep) 根据闭包 is_sep 去定切分的位置
    // splitn(n , is_sep) 拆分为 n 段
    // chunks(n) 分为长度为 n 的块
    let mut iter1 = vec1.chunks(2);
    println!("{:?}", iter1.next().unwrap()); // [11, 2]
    // windows(n) 滑动窗口

    // 交换
    vec1.swap(1, 2);
    // a.swap_with_slice(b) 互换内容
    // vec.swap_remove(i) 交换后移除

    // 填充
    // slice.fill()
    // slice.fill_with(function)

    // 排序与搜索
    let mut ages = vec![18, 23, 14, 26, 80, 50];
    // 升序排列
    ages.sort();
    println!("ages: {:?}", ages); // ages: [14, 18, 23, 26, 50, 80]
    // slice.sort_by(cmp) - 按 cmp 回调排序
    let mut names = vec!["张三", "李四", "王五", "郑和", "辛弃疾"];
    names.sort_by(|a, b| a.cmp(b));
    println!("{:?}", names); // ["张三", "李四", "王五", "辛弃疾", "郑和"]

    // 逆转
    names.reverse();

    // 二分搜索
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let result = s.binary_search(&13);
    println!("{:?}", result.ok()); // Some(9)

    // 包含
    // contains(&value)

    // starts_with() 以什么开头
    // ends_with() 以什么结尾

    // 随机元素
    // 需要使用 rand crate
    // 从向量中获取随机
    let mut vec3 = vec![0, 3, 5, 7, 9, 6];
    let mut rng = thread_rng();
    let a = vec3.choose(&mut rng);
    println!("{}", a.unwrap()); 
    // 随机洗牌
    vec3.shuffle(&mut rng);
    println!("vec3: {:?}", vec3); // vec3: [9, 3, 6, 5, 7, 0]
}