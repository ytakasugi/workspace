//use std::mem;

fn main() {
    let color = "green";

    // `color`をプリントするためのクロージャ。
    // これは`color`を借用(`&`)し、その借用とクロージャを`print`という名の変数に保持する。
    // 借用は`print`がスコープから出るまで続く。
    // `println!`は参照を与えれば機能するので、これ以上なにかする必要はない。
    let print = || println!("color: {}", color);

    // printを呼び出す
    print();

    // `color`は再度不変の参照をできる
    // `color`への不変参照
    let reborrow = &color;
    println!("reborrow: {}", reborrow);
    print();
}
