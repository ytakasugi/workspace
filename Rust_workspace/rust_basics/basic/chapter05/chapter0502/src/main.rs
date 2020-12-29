fn main() {
    // Vec<bool>型
    let v1 = vec![false, true, false];
    // ベクタv1の中身を表示
    for i in v1.iter() {
        println!("v1 = {}", i);
    }
    // Vec<f64>型
    let v2 = vec![0.0, -1.0, 1.0, 0.5];
    println!("v2.length = {}", v2.len());
    // ベクタv1の中身を表示
    for j in v2.iter () {
        println!("v2 = {}", j);
    }
}
