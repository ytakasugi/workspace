// 長いライフタイムは、短いものに圧縮(coerce)することで、そのままでは動作しないスコープの中でも使用できるようになります。

// ここではRustはライフタイムを出来る限り短く見積もり、
// 2つの参照をそのライフタイムに押し込める
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>`は「ライフタイム`'a`は最低でも`'b`と同じ長さ」と読める。
// ここでは、`&'a i32`をとり、`&'b i32`に圧縮して返す。
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}


fn main() {
    // Longer lifetime
    let first = 2;

    {
        // Shorter lifetime
        let second = 3;

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}
