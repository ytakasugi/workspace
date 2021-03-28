fn main() {
    let init = 0;
    let sum = (1 ..= 5).fold(init, |acc, x| acc + x);  // (((((0 + 1) + 2) + 3) + 4) + 5)
    println!("{}", sum);

    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());

    assert_eq!(maybe_some_len, Some(13));
    println!("{}", maybe_some_len.unwrap());

    let line = "1\n2\n3\n4\n";

// `Ok(n)`の場合、`i * 2`を返し、`Err(e)`の場合は何も返さない
for num in line.lines() {
    match num.parse::<i32>().map(|i| i * 2) {
        Ok(n) => println!("{}", n),
        Err(..) => {}
    }
}

// `x`を引数とし、`x`に2乗を返却するクロージャ
let c = |x| {
    x * x
};

println!("x is {}", c(9));

println!("b is {}", b'0');

}