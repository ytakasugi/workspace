use std::env;
use std::fs::File;
//  std::io::preludeモジュールは、IO 周りでよく使う型のインポートを楽にするためのモジュールである
use std::io::prelude::*;

fn main() {
    // イテレータをベクタに変換
    // `collect()`を使用するときは、コンパイラは型を推論できないないので型注釈すること
    let args: Vec<String> = env::args().collect();

    // 引数として、`args`を参照する
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    //let mut f = File::open(config.filename).expect("File not found");
    //let mut contents = String::new();
    //f.read_to_string(&mut contents).expect("something went wrong reading the file");
    
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough argments.");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        Config{ query, filename}

    }
}

