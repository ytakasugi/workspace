fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a + std::fmt::Debug {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let v =vec![1, 2, 3];
    println!("{:?}", double_positives(&v))
}
