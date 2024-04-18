use std::convert::{ TryFrom, TryInto };

// 整型
fn main() {
    let age: u8 = 18;
    println!("age is {}", age); 
    // age is 18

    // 返回类型的范围
    println!("u8 range: {} ~ {}", u8::MIN, u8::MAX); 
    // u8 range: 0 ~ 255

    // 字节字面量
    #[allow(non_snake_case)]
    let A = b'A';
    println!("A is {}", A);

    // 整型之间的转换
    let a: i32 = 300;
    let b: u8 = a as u8;
    let c: i64 = a as i64;

    println!("i32: {}", a); // 300
    println!("u8: {}", b);  // 44, 转换为 u8 类型，只有值在 u8 的范围内才安全
    println!("i64: {}", c); // 300

    let d: i32 = 300;
    // try_from
    let f = u32::try_from(d).expect("数值超出了 u8 的范围");
    println!("f: {}", f); // f: 300
    let e = u8::try_from(d).expect("数值超出了 u8 的范围");
    println!("e: {}", e); // 数值超出了 u8 的范围: TryFromIntError(())

    // try_into 
    let g: u32 = d.try_into().expect("数值超出了 u8 的范围");
    println!("g: {}", g); // g: 300
    let h = u8::try_from(d).expect("数值超出了 u8 的范围");
    println!("h: {}", h); // 数值超出了 u8 的范围: TryFromIntError(())
    

}