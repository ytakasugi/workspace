fn main() {
    let v = [10, 40, 30];
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(!v.starts_with(&[50]));
    assert!(!v.starts_with(&[10, 50]));

    println!("{}", v.starts_with(&[10]));
    println!("{}", v.starts_with(&[40]));
    println!("{}", v.starts_with(&[50]));

    let a = 0;
    let b = &a;
    println!("var: {}", a);
    println!("borrow: {}", b);

    let mut c = 0;
    let d = &mut c;
    //println!("mut_var: {}", c);
    println!("mut_borrow: {}", d);
}