use std::io;

fn main() {
    // 長さ入力する
    println!("What is the vartical length?:");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Faild to read line");
    
}
