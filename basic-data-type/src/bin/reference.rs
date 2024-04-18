// 引用
fn main() {
  let mut a = 10;
  let b = &a;
  println!("b: {}", b); // 10
  println!("a: {}", a); // 10

  let c = &mut a;
  *c = 11; // 11
  println!("c: {}", a);
}