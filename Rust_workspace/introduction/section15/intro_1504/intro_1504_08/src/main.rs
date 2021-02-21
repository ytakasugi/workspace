// 'staticライフタイムは全てのライフタイムの中で最長で、プログラムが動作している間、常に有効になります。
// 'staticであっても、より短いライフタイムに圧縮されることはあります。
// 'staticなライフタイムをもつ変数を作成する方法は2つあり、いずれも実行バイナリの一部としてROM上に保存されます。
// 
//   ・static宣言とともに定数を作成する。
//   ・文字列リテラル で&'static str型を持つ変数を作成する。

// `'static`ライフタイムを持つ定数を作成
static NUM: i32 = 18;

// `NUM`への参照を返す。ライフタイムは`'static`から引数のライフタイムへと圧縮されている。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // 文字列リテラルを用いて変数を作成し、プリントする
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // `static_string`がスコープから抜けると、参照は使用することができなくなるが、データはバイナリ中に残る。
    }

    {
        // `coerce_static`関数を呼び出すために、整数を作成
        let lifetime_num = 9;

        // `NUM`を`lifetime_num`のライフタイムへと圧縮
        let coerce_static = coerce_static(&lifetime_num);

        println!("coerce_static: {}", coerce_static);
    }
    println!("NUM: {} stays accessible!", NUM);
}
