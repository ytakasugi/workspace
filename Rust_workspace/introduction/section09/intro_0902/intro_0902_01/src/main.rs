//use std::mem;

fn main() {
    let color = "green";

    // `color`をプリントするためのクロージャ。
    // これは`color`を借用(`&`)し、その借用とクロージャを`print`という名の変数に保持する。
    // 借用は`print`がスコープから出るまで続く。
    // `println!`は参照を与えれば機能するので、これ以上なにかする必要はない。
    let print = || println!("color: {}", color);

    // printを呼び出す
    print();

    // `color`は再度不変の参照をできる
    // `color`への不変参照
    let reborrow = &color;
    println!("reborrow: {}", reborrow);
    print();

    // moveや再借用は`print`最終使用後に許可される
    let color_moved = color;
    println!("color_moved: {}", color_moved);

    // `count`をインクリメントするためのクロージャ。
    // `count`と`&mut count`の両方を取ることができるが、後者のほうが制限が少ないため、そちらを取る。直後に`count`を借用する。
    // （訳注: `count`だと`&mut count`と違い、一度しか呼ぶことができない。）
    //
    // `increment`には`mut`をつける必要がある。
    // なぜならミュータブルな型が中で使用されているからである。ミュータブルなクロージャは呼ぶたびに内部変数を変更する。
    let mut count = 0;

    let mut increment = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // クロージャを実行
    increment();

    // このクロージャは後から呼び出されるため、`count`を可変借用している
    // 再借用しようとすると、エラーとなる
    // let _reborrow = &count;
    increment();

    // クロージャは、もはや`&mut count` を借用する必要がなくなりました。
    // したがって、エラーなく再借用することができます
    // let count_reborrowed = &mut count;
    let count_reborrowed = &count;

    println!("count_reborrowed: {}", count_reborrowed);

}
