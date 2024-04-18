// 浮点类型
fn main() {
    let f1: f64 = 1.03;
    let f2: f32 = 1.03;
    assert_eq!(f1 as f32, f2); // 通过
    assert_eq!(f1, f2.into()); // assertion `left == right` failed
}