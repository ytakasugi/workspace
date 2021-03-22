fn main() {
    let init = 0;
    let sum = (1 ..= 5).fold(init, |acc, x| acc + x);  // (((((0 + 1) + 2) + 3) + 4) + 5)
    println!("{}", sum);

    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());

    assert_eq!(maybe_some_len, Some(13));
    println!("{}", maybe_some_len.unwrap());
}