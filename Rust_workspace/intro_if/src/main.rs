use std::io;

fn main() {
    println!("Please enter a n");

    let n = get_input();

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        print!("{} is zero", n)
    }

    
}

fn get_input() -> i32 {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok();
    return n.trim().parse().ok().unwrap();
}
