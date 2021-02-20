#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str`はread-onlyメモリ上の文字列への参照
    author: &'static str,
    title: &'static str,
    year: u32,
}

// この関数はBook型へのリファレンスを取る。
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// この関数はミュータブルなBook型へのミュータブルなリファレンスを取り、`year`を2014へ変更する。
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // `immutabook`という名のイミュータブルなBookを作成
    let immutabook = Book {
        // 「"」で囲まれた部分は`&'static str`型になる。
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // `immutabook`のミュータブルなコピーを作成し、`mutabook`を作成
    let mut mutabook = immutabook;

    // immutableなオブジェクトをimmutableに借用
    borrow_book(&immutabook);

    // mutableなオブジェクトをimmutableに借用
    borrow_book(&mutabook);

    // mutableなオブジェクトをmutableに借用
    new_edition(&mut mutabook);

    // エラー！イミュータブルなオブジェクトをミュータブルに借用することはできない
    // new_edition(&mut immutabook);
}