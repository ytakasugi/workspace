use std::env;
use std::fs::File;
//  std::io::preludeモジュールは、IO 周りでよく使う型のインポートを楽にするためのモジュールである
use std::io::prelude::*;

fn main() {
    // イテレータをベクタに変換
    // `collect()`を使用するときは、コンパイラは型を推論できないないので型注釈すること
    let args: Vec<String> = env::args().collect();

    // ベクタの1番目の要素は、バイナリ名
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
    
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
