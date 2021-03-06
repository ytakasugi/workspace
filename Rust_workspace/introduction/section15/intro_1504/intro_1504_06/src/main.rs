// ジェネリック型に境界(bound)を与え、特定のトレイトを実装していることを保証できるのと同様、
// ライフタイム（それ自身ジェネリック型）にも境界を与えることができます。
// ここでは多少異なる意味を持ちますが`+`は同じです。以下の構文の意味をチェックしてください。
//
//   1.T: 'a: T内の 全ての 参照は'aよりも長生きでなくてはならない
//   2.T: Trait + 'a: 上に加えてTはTraitという名のトレイトを実装してなくてはならない。
//
// 上記の構文を実際に動く例で見ていきましょう。whereキーワードの後に注目してください。

// ライフタイムを紐付けるトレイト
use std::fmt::Debug;

// `Ref`は`'a`というライフタイムを持つジェネリック型`T`に対する参照を持ち、
// `T`の値に対する参照は必ず`'a`よりも長生きでなくてはならない。
// さらに、`Ref`のライフタイムは`'a`を超えてはならない。
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

// `Debug`トレイトを利用してプリントを行うジェネリック関数
fn print<T>(t: T)
where
    T: Debug
{
    println!("`print`: t is {:?}", t);
}

// `Debug`を実装している`T`への参照を取る。
// `T`への参照は必ず`'a`よりも長生きでなくてはならない。
// さらに、`'a`は関数自体よりも長生きでなくてはならない。
fn print_ref<'a, T>(t: &'a T) 
where
    T: Debug + 'a
{
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
