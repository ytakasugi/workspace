fn main() {
    let init = 0;
    let sum = (1 ..= 5).fold(init, |acc, x| acc + x);  // (((((0 + 1) + 2) + 3) + 4) + 5)
    println!("{}", sum)
}