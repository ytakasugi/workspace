use std::io;

fn main() {
    // 変数の宣言
    let a = input();
    let b = input();

    let ans = if (a * b) % 2 == 0 {
        "Even"
    } else {
        "Odd"
    };

    println!("{}", ans);
}

// 標準入力用の関数の作成
fn input() -> u32 {
    let mut number = String::new();
    io::stdin().read_line(&mut number).ok();
    return number.trim().parse().ok().unwrap();
}
