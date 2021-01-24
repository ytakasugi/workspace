use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argments.");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{ query, filename})

    }
}

// 引数にConfigインスタンスを取る
// Resultが`Ok`の場合は`ユニット型()`、エラーの場合は`Error`トレイトを返す
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    // 空のString型を作成
    let mut contents = String::new();
    // すべてのバイトを読み込みバッファに追加する
    f.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
    //`ユニット型()`を`Ok()`で包む
    Ok(())
} 