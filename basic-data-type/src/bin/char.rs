// char
#[allow(unused_variables)]
fn main() {
    let letter: char = 'a';   // 英文字母
    let number: char = '1';   // 数字字符
    let symbol: char = '$';   // 符号
    let space: char = ' ';    // 空格
    let emoji: char = '😂';  // 表情符号
    let chinese: char = '中'; // 中文字符

    // 一些 char 类型的操作
    println!("{} is alphabetic: {}", letter, letter.is_alphabetic()); // a is alphabetic: true
    println!("{} is digit: {}", number, number.is_digit(10));         // 1 is digit: true
    println!("{} to uppercase: {}", letter, letter.to_uppercase()); // a to uppercase: A
}

