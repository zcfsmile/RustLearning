// 元组
#[allow(unused_variables)]
fn main() {
    let temp = (); // 零元组
    let one = (100, );
    let two = (100, 200);
    println!("{}", two.0); // 100

    let text = "Hello World";
    // 元组解构
    let (hello, world) = text.split_at(5);
    println!("hello: {}, world: {}", hello, world); // hello: Hello, world:  World
}
