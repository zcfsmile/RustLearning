use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    // 插入
    map.insert("john", 98);
    // 长度
    println!("len: {}", map.len()); // len: 1
                                    // 是否为空
    println!("is_empty: {}", map.is_empty()); // is_empty: false
                                              // 获取值
    println!("score: {:?}", map.get("john")); // score: Some(98)
                                              // 是否包含某个键
    println!("is containe: {}", map.contains_key("john")); // is containe: true
                                                           // 移除
    map.remove("john");

    let mut map1 = HashMap::new();
    map1.insert("frank", 100);
    map1.insert("lucy", 89);
    map.extend(map1);
    println!("map: {:?}", map); // map: {"lucy": 89, "frank": 100}

    // 清空
    map.clear();
    println!("map: {:?}", map); // map: {}

    // Entry
    // 如果不存在 peter key,就创建一个默认值
    map.entry("peter").or_default();
    println!("map: {:?}", map); // map: {"peter": 0}
    map.entry("poll").or_insert(67);
    println!("map: {:?}", map); // map: {"poll": 67, "peter": 0}

    // 对 map 进行迭代
    println!("keys: {:?}", map.keys()); // keys: ["peter", "poll"]
    println!("values: {:?}", map.values()); // values: [0, 67]

    for (k, v) in map {
        println!("{}: {}", k, v);
    }
    // peter: 0
    // poll: 67
}
