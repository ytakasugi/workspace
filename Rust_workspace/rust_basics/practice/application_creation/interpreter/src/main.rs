fn main() {
    println!("Hello, world!");
}

// 字句解析器が出力する字句を Token 列挙体として定義
enum Token {
    // 数字
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lparen,
    Rparen,
}

// 字句解析器を構造体Lexerとして定義
// Lexer は初期化時にソースコードの文字列を受け取る
// 字句解析の実行は Lexer::token 関数で実施する。
struct Lexer {
    // 入力された文字列は、`Vec<char>`とする
    input: Vec<char>,
    // 解析中のインデックス
    potition: usize,
}

// 構造体Lexerのメソッドを定義
impl Lexer {
    // 初期化用の関連関数
    fn new(input: Vec<char>) -> Lexer {
        Lexer {input, potition: 0}
    }

    fn token(&mut self) -> Option<Token> {
        // 現在解析中の文字を字句として取得し、インデックスを一つ進める
        use std::iter::FromIterator;
        // 空白をスキップする
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }

        // 解析中の文字を取得して字句に変換する
        let curr = self.curr()?;
        let token = if Self::is_number(curr) {
            let mut number = vec![*curr];
            while self.peek().is_some() && Self.is_number(self.peek().unwrap()) {
                self.next();
                number.push(*self.curr().unwrap());
            }
        }
    }
}
