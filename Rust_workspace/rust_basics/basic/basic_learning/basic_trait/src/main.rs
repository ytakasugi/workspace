// `trait トレイト名`でトレイトを定義
// DuckLikeトレイトを定義する
trait DuckLike {
    // トレイトを実装すべき型が実装すべきメソッドを定義
    fn quack(&self);

    // デフォルトメソッドを定義することもできる
    fn walk(&self) {
        println!("walking!");
    }
}

// トレイトを実装するためだけのデータ型にはUnit構造体が便利
struct Duck;

// `impl トレイト名 for 型名 {..}で定義可能
impl DuckLike for Duck {
    // トレイトで実装されていないメソッドを実装側で定義する
    fn quack(&self) {
        println!("quack");
    }
}

struct Tsuchinoko;

// 別の型も定義できる
impl DuckLike for Tsuchinoko {
    fn quack(&self) {
        // どうやらツチノコの正体は猫だったようだ
        println!("mew");
    }

    // デフォルトメソッドを上書きすることもできる
    fn walk(&self) {
        println!("wriggling");
    }
}

// 既存の方にトレイトを実装することもできる
// モンキーパッチをしているような気分
impl DuckLike for i64 {
    fn quack(&self) {
        for _ in 0..*self {
            println!("quack");
        }
    }
}

fn main() {
    let duck = Duck;
    let tsuchinoko = Tsuchinoko;
    let i = 3;
    // Duckトレイトのquackメソッドを呼び出している
    duck.quack();
    // Tsuchinokoトレイトのquackメソッドを呼び出している
    tsuchinoko.quack();
    // Tsuchinokoトレイトのwalkメソッドを呼び出している
    Tsuchinoko.walk();
    // 既存の方に実装したquackメソッドを呼び出している
    i.quack();
}
