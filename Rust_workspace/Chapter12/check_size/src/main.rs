/*use std::io;

// 入力されたString型を数値へ変換する関数
fn get_input() -> u32 {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok();
    return n.trim().parse().ok().unwrap();
}*/

fn main() {
    /*println!("please enter the height");
    let height = get_input();
    println!("please enter the width");
    let width = get_input();
    println!("please enter the depth");
    let depth = get_input();*/

    let size = Checksize { height: 50, width: 60, depth:90 };
    // let size = height + width + depth;
    println!("message {}", size.check(&size));

    struct Checksize {
        height: u32,
        width: u32,
        depth: u32,
    }

    impl Checksize {
        fn check(&self, other: &Checksize) -> bool {
            180 >= other.height + other.width + other.depth
        }
    }
}
