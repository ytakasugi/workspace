
// 位置情報。.0から.1までの区間を表す。
// たとえば、Loc(4, 6)なら入力文字の5文字目から7文字目までの区間を表す(0始まり)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

// locにメソッドを定義しておく
impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

// アノテーション。値に様々なデータをもたせたもの。ここでは、`Loc`をもたせる
// ジェネリクス構造体Annotを定義
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

// impl<T>は、型Annot<T>にメソッドを定義していることを指定している。
impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
        // [0-9][0-9]*
        Number(u64),
        // +
        Plus,
        // -
        Minus,
        // *
        Asterisk,
        // /
        Slash,
        // (
        LParen,
        // )
        RParen,
}

// `TokenKind` にアノテーションをつけたものを `Token` として定義しておく
type Token = Annot<TokenKind>;

// ヘルパーメソッドを定義しておく
impl Token {
    fn number(n:u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }

    fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }

    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }

    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

// `TokenKind` と同様に字句解析エラーを実装をする
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}


fn main() {
    println!("Hello, world!");
}
