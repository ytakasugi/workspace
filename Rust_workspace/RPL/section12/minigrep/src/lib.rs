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


// 空のベクタを返す関数
// search関数に返される値は、search関数にcontents引数で渡されているデータと同じライフタイムを持つ
// スライスに参照されるデータは、参照が有効になるために有効である必要がある
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // linesメソッドは、イテレータを返す
    for line in contents.lines() {
        // クエリを求めて各行を検索する
        if line.contains(query) {
            // 合致した行を保存する
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}


