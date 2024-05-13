fn main() {
    let str_slices = "Hello";
    // 字符串切片生成字符迭代器
    let h_char = str_slices.chars().next().unwrap();
    println!("h_char: {}", h_char); // h_char: H

    // 字符分类
    let test_char = '2';
    // 1. 是否为数值类型
    println!("{}", test_char.is_numeric()); // true
    // 2. 是否为字母
    println!("{}", test_char.is_alphabetic()); // false
    // 3. 是否为数值或字母
    println!("{}", test_char.is_alphanumeric()); // true
    // 4. 是否为空白字符
    println!("{}", test_char.is_whitespace()); // false
    // 5. 是否为控制字符
    println!("{}", test_char.is_control()); // false

    // char 的 ASCII 分类方法
    // 方法前缀为 is_ascii_
    println!("{}", test_char.is_ascii()); // true
    println!("{}", test_char.is_ascii_digit()); // true

    // 处理数字
    // 转数字
    let char_a = 'a';
    let char_2 = '2';
    println!("{:?}", char_a.to_digit(10)); // None
    println!("{:?}", char_2.to_digit(10)); // Some(2)
    // 数字转字符
    println!("{:?}", std::char::from_digit(15, 16)); // Some('f')

    // 字符大小写转换
    let mut upper = 'a'.to_uppercase();
    println!("{:?}", upper.next()); // Some('A')
    println!("{:?}", upper.next()); // None
    // 转消息 to_lowercase()
    // 是否是小写 is_lowercase()
    // 是否是大写 is_uppercase()

    // 与整数的转换
    // as 会将char转换为任何整数类型，并抹掉高位；
    // as 会将任何 u8 值转换为 char。
}