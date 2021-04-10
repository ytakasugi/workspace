### [補足時の型推論](https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/input_parameters.html)

Rustは、ほとんどの場合型アノテーションなしで変数をその場でキャプチャする方法を選択しますが、関数を作成する場合、このあいまいさは許容されません。クロージャを入力パラメータとして使用する場合、クロージャの完全な型には、いくつかの特性の1つを使用して注釈を付ける必要があります。制限が小さい順に、次のとおりです。

- `Fn`：クロージャは参照によってキャプチャします（`＆T`）

- `FnMut`：クロージャは可変参照（`＆mut T`）によってキャプチャします

- `FnOnce`：クロージャは値（`T`）でキャプチャします

 変数ごとに、コンパイラーは可能な限り制限の少ない方法で変数をキャプチャします。


たとえば、`FnOnce`として注釈が付けられたパラメーターについて考えてみます。これは、クロージャーが`＆T`、`＆mut T`、または`T`によってキャプチャーできることを指定しますが、コンパイラーは、キャプチャーされた変数がクロージャーでどのように使用されるかに基づいて最終的に選択します。


これは、移動が可能であれば、あらゆるタイプの借用も可能である必要があるためです。逆は当てはまらないことに注意してください。パラメータに`Fn`アノテーションが付けられている場合、`＆mutT`または`T`による変数のキャプチャは許可されません。


次の例では、`Fn`、`FnMut`、および`FnOnce`の使用法を入れ替えて、何が起こるかを確認してください。



~~~rust
// クロージャを引数に取り、それを呼び出す関数。

// <F>はFが "汎用型パラメータ "であることを示します。

fn apply<F>(f: F) where

  // クロージャには引数も返り値もない。

  F: FnOnce() {

  // ^ TODO: ここを`Fn`あるいは`FnMut`に変えてみましょう。



  f();

}



// クロージャを引数に取り、`i32`を返す関数

fn apply_to_3<F>(f: F) -> i32 where

  // このクロージャは引数、返り値ともに`i32`

  F: Fn(i32) -> i32 {



  f(3)

}



fn main() {

  use std::mem;



  let greeting = "hello";

  // A non-copy type.

  // コピーではなくmoveが起きる型

  let mut farewell = "goodbye".to_owned();



  // 変数を2つ補足。`greeting`は参照を、

  // `farewell`は値をそれぞれ捕捉する。

  let diary = || {

    // `greeting` is by reference: requires `Fn`.

    // `greeting`は参照なので、`Fn`が必要。

    println!("I said {}.", greeting);



    // `farewell`の値を変更するので、この時点で`FnMut`が必要になる。

    farewell.push_str("!!!");

    println!("Then I screamed {}.", farewell);

    println!("Now I can sleep. zzzzz");

    

    // `mem::drop`を明示的に呼ぶと`farewell`が値で

    // 捕捉される必要性が発生する。よって`FnOnce`が必要になる。

    mem::drop(farewell);

  };



  // クロージャを適用する関数を実行。

  apply(diary);



  // double` は `apply_to_3` の特徴量を満たす。

  let double = |x| 2 * x;



  println!("3 doubled: {}", apply_to_3(double));

}

~~~



---

### [クロージャを返す関数](https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/output_parameters.html)



入力パラメーターとしてクロージャを返すことが可能であるため、出力パラメーターとしてクロージャを返すことも可能である必要があります。 ただし、匿名のクロージャタイプは定義上不明であるため、それらを返すには`impl Trait`を使用する必要があります。


クロージャを返すための有効なトレイトは次のとおりです。



- `Fn`

- `FnMut`

- `FnOnce`



これに加えて、`move`キーワードを使用する必要があります。これは、すべてのキャプチャが値によって発生することを示します。 これが必要なのは、関数が終了するとすぐに参照によるキャプチャが削除され、クロージャに無効な参照が残るためです。



~~~rust
fn create_fn() -> impl Fn() {

  let text = "Fn".to_owned();



  move || println!("This is a: {}", text)

}



fn create_fnmut() -> impl FnMut() {

  let text = "FnMut".to_owned();



  move || println!("This is a: {}", text)

}



fn create_fnonce() -> impl FnOnce() {

  let text = "FnOnce".to_owned();



  move || println!("This is a: {}", text)

}



fn main() {

  let fn_plain = create_fn();

  let mut fn_mut = create_fnmut();

  let fn_once = create_fnonce();



  fn_plain();

  fn_mut();

  fn_once();

}

~~~

---

### [New type Idiom](https://doc.rust-jp.rs/rust-by-example-ja/generics/new_types.html)


newtypeイディオムは、正しい型の値がプログラムに与えられることをコンパイル時に保証します。


例えば、年齢を年単位でチェックする年齢確認関数には、Years型の値を与えなければなりません。



~~~rust
struct Years(i64);



struct Days(i64);



impl Years {

  pub fn to_days(&self) -> Days {

    Days(self.0 * 365)

  }

}





impl Days {

  // 年数の一部を切り捨てます。

  pub fn to_years(&self) -> Years {

​    Years(self.0 / 365)

  }

}



fn old_enough(age: &Years) -> bool {

  age.0 >= 18

}



fn main() {

  let age = Years(5);

  let age_days = age.to_days();

  println!("Old enough {}", old_enough(&age));

  println!("Old enough {}", old_enough(&age_days.to_years()));

  // println!("Old enough {}", old_enough(&age_days));

}

~~~

最後のprint文のコメントを外して、指定された型がYearsでなければならないことを確認してください。

newtypeの値を基底型として取得するには、次のようなタプル構文を使用することができます。



~~~rust
struct Years(i64);



fn main() {

  let years = Years(42);

  let years_as_primitive: i64 = years.0;

}

~~~

---

### [Destructor](https://doc.rust-jp.rs/rust-by-example-ja/scope/raii.html#destructor)


Rustのデストラクタの概念は Dropトレイトで提供されています。デストラクタはリソースがスコープ外になったときに呼び出されます。この特性はすべての型に実装する必要はありませんが、独自のデストラクタロジックが必要な場合にのみ実装してください。

以下の例を実行して、Dropトレイト がどのように動作するかを確認してください。メイン関数の変数がスコープ外になると、カスタムデストラクタが呼び出されます。



```rust
struct ToDrop;



impl Drop for ToDrop {

  fn drop(&mut self) {

    println!("ToDrop is being dropped");

  }

}



fn main() {

  let x = ToDrop;

  println!("Made a ToDrop!");

}



```

---

### [Returning Traits with `dyn`](https://doc.rust-jp.rs/rust-by-example-ja/trait/dyn.html#returning-traits-with-dyn)


`Rust`コンパイラは、各関数の戻り値の型がどれだけのスペースを必要とするかを知る必要があります。つまり、すべての関数は具体的な型を返さなければなりません。他の言語とは異なり、`Animal`のようなトレイトを持っている場合、`Animal`を返す関数を書くことはできません。


しかし、簡単な回避策があります。トレイトオブジェクトを直接返すのではなく、`Animal`を含む`Box`を返すようにします。`Box`はヒープ内のメモリへの参照にすぎません。参照は静的に既知のサイズを持ち、コンパイラはそれがヒープに割り当てられた`Animal`を指していることを保証できるので、関数からトレイトを返すことができます。


`Rust`は、ヒープ上のメモリを確保する際には、可能な限り明示的にしようとします。そのため、もしあなたの関数がこの方法でヒープ上のトレイトへのポインタを返すのであれば、戻り値の型を`dyn`キーワードで書く必要があります。



~~~rust
struct Sheep {}

struct Cow {}



trait Animal {

  // インスタンスメソッドシグネチャ

  fn noise(&self) -> &'static str;

}



// `Sheep`構造体に`Animal`トレイトを実装する

impl Animal for Sheep {

  fn noise(&self) -> &'static str {

    "baaaaah!"

  }

}



// `Cow`構造体に`Animal`トレイトを実装する

impl Animal for Cow {

  fn noise(&self) -> &'static str {

    "moooooo!"

  }

}



// `Animal`を実装した構造体を返しますが、コンパイル時にどの構造体かはわかりません。

fn random_animal(random_number: f64) -> Box<dyn Animal> {

  if random_number < 0.5 {

    Box::new(Sheep {})

  } else {

    Box::new(Cow {})

  }

}



fn main() {

  let random_number = 0.234;

  let animal = random_animal(random_number);

  println!("You've randomly chosen an animal, and it says {}", animal.noise());

}

~~~

---

### [`impl Trait`](https://doc.rust-jp.rs/rust-by-example-ja/trait/impl_trait.html#impl-trait)


関数が`MyTrait`を実装した型を返す場合、その戻り値の型を`-> impl MyTrait`と書くことができます。これは、型のシグネチャを非常に簡単にするのに役立ちます。


~~~rust
use std::iter;

use std::vec::IntoIter;



// この関数は2つの `Vec<i32>` を結合し、その上のイテレータを返します。

// その戻り値の型がどれほど複雑か見てください!

fn combine_vecs_explicit_return_type(

  v: Vec<i32>,

  u: Vec<i32>,

) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {

  v.into_iter().chain(u.into_iter()).cycle()

}



// これは全く同じ関数ですが、戻り値の型は `impl Trait` を用います。

// 見てください、こんなにシンプルになりました

fn combine_vecs(

  v: Vec<i32>,

  u: Vec<i32>,

) -> impl Iterator<Item=i32> {

  v.into_iter().chain(u.into_iter()).cycle()

}



fn main() {

  let v1 = vec![1, 2, 3];

  let v2 = vec![4, 5];

  let mut v3 = combine_vecs(v1, v2);

  assert_eq!(Some(1), v3.next());

  assert_eq!(Some(2), v3.next());

  assert_eq!(Some(3), v3.next());

  assert_eq!(Some(4), v3.next());

  assert_eq!(Some(5), v3.next());

  println!("all done");

}

~~~


さらに重要なことに、`Rust`の型の中には書き出すことができないものがあります。例えば、すべてのクロージャは、それ自身の名前のない具象型を持っています。`impl Trait`構文の前は、クロージャを返すためにヒープ上で割り当てなければなりませんでした。しかし、現在では、以下のように静的にすべてを行うことができます。



~~~rust
// Returns a function that adds `y` to its input

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {

  let closure = move |x: i32| { x + y };

  closure

}



fn main() {

  let plus_one = make_adder_function(1);

  assert_eq!(plus_one(2), 3);

}

~~~


また、マップやフィルタのクロージャを使用するイテレータを返すために`impl Trait`を使用することもできます！これにより、マップやフィルタの使用がより簡単になります。クロージャの型には名前がないので、関数がクロージャを使用したイテレータを返す場合、明示的な戻り値の型を書き出すことはできません。しかし`impl Trait`を使えば、これを簡単に行うことができます。



~~~rust
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {

  numbers

    .iter()

    .filter(|x| x > &&0)

    .map(|x| x * 2)

}

~~~

---

### [Supertraits](https://doc.rust-jp.rs/rust-by-example-ja/trait/supertraits.html#supertraits)


Rustには「継承」はありませんが、他の`trait`のスーパーセットとして trait を定義することができます。例えば以下のように


~~~rust
trait Person {

  fn name(&self) -> String;

}



// StudentはPersonのスーパートレイトです。

// Studentを実装するには、Personも実装する必要があります。

trait Student: Person {

  fn university(&self) -> String;

}



trait Programmer {

  fn fav_language(&self) -> String;

}



// CompSciStudent(computer science student)は、ProgrammerとStudentの両方のスーパイトレイトです。

// CompSciStudentを実装するには、両方のサブトレイトを実装する必要があります。

trait CompSciStudent: Programmer + Student {

  fn git_username(&self) -> String;

}



fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {

  format!(

    "My name is {} and I attend {}. My Git username is {}",

    student.name(),

    student.university(),

    student.git_username()

  )

}



fn main() {}

~~~



- See also

 [スーパートレイト](https://doc.rust-jp.rs/book-ja/ch19-03-advanced-traits.html#%E3%82%B9%E3%83%BC%E3%83%91%E3%83%BC%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%97%E3%81%A6%E5%88%A5%E3%81%AE%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E5%86%85%E3%81%A7%E3%81%82%E3%82%8B%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E6%A9%9F%E8%83%BD%E3%82%92%E5%BF%85%E8%A6%81%E3%81%A8%E3%81%99%E3%82%8B)

---

### [Disambiguating overlapping traits](https://doc.rust-jp.rs/rust-by-example-ja/trait/disambiguating.html#disambiguating-overlapping-traits)

1つの型には多くの異なるトレイトを実装することができます。2つのトレイトが両方とも同じ名前を必要とする場合はどうすればよいでしょうか？例えば、多くのトレイトは`get()`という名前のメソッドを持っているかもしれません。また、異なる戻り値の型を持つこともあるかもしれません。



良いニュースです: それぞれの `trait`の実装には独自の`impl`ブロックがあるので、どの`trait`の`get`メソッドを実装しているのかが明確になります。



では、これらのメソッドを呼び出すときはどうすればいいのでしょうか？これらのメソッドを区別するために、完全修飾構文を使用しなければなりません。



~~~rust
trait UsernameWidget {

  // このウィジェットから選択されたユーザ名を取得する

  fn get(&self) -> String;

}



trait AgeWidget {

  // このウィジェットの選択された年齢を取得するこのウィジェットの選択された年齢を取得する

  fn get(&self) -> u8;

}



// UsernameWidget と AgeWidget の両方を持つフォーム

struct Form {

  username: String,

  age: u8,

}



impl UsernameWidget for Form {

  fn get(&self) -> String {

    self.username.clone()

  }

}



impl AgeWidget for Form {

  fn get(&self) -> u8 {

    self.age

  }

}



fn main() {

  let form = Form{

    username: "rustacean".to_owned(),

    age: 28,

  };



  // If you uncomment this line, you'll get an error saying 

  // "multiple `get` found". Because, after all, there are multiple methods

  // named `get`.

  // println!("{}", form.get());



  let username = <Form as UsernameWidget>::get(&form);

  assert_eq!("rustacean".to_owned(), username);

  let age = <Form as AgeWidget>::get(&form);

  assert_eq!(28, age);

}

~~~



- See also


 [Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name](https://doc.rust-jp.rs/book-ja/ch19-03-advanced-traits.html#%E6%98%8E%E7%A2%BA%E5%8C%96%E3%81%AE%E3%81%9F%E3%82%81%E3%81%AE%E3%83%95%E3%83%AB%E3%83%91%E3%82%B9%E8%A8%98%E6%B3%95-%E5%90%8C%E3%81%98%E5%90%8D%E5%89%8D%E3%81%AE%E3%83%A1%E3%82%BD%E3%83%83%E3%83%89%E3%82%92%E5%91%BC%E3%81%B6)

---

### [リテラルとエスケープ](https://doc.rust-jp.rs/rust-by-example-ja/std/str.html#literals-and-escapes)

特殊文字を含む文字列リテラルの書き方は複数あります。どれも似たような`&str`になるので、書きやすい形式を使うのが一番です。同様に、バイト文字列リテラルにも複数の書き方がありますが、いずれも`&[u8; N]`となります。



一般的に特殊文字はバックスラッシュでエスケープされます。\. この方法では、印刷できない文字や入力方法がわからない文字も含めて、あらゆる文字を文字列に追加することができます。文字通りのバックスラッシュが必要な場合は、別のバックスラッシュでエスケープします。`\\`



リテラルの中にある文字列や文字リテラルの区切り文字はエスケープしなければなりません。`"\""`, `'\''`.

~~~rust
fn main() {

  // エスケープを使ってバイトを16進数で書くこともできますが...。

  let byte_escape = "I'm writing \x52\x75\x73\x74!";

  println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);



  // ...またはUnicodeコードポイントでバイトを記述することができます。

  let unicode_codepoint = "\u{211D}";

  let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";



  println!("Unicode character {} (U+211D) is called {}",

        unicode_codepoint, character_name );





  let long_string = "String literals

            can span multiple lines.

            The linebreak and indentation here ->\

            <- can be escaped too!";

  println!("{}", long_string);

}

~~~

エスケープしなければならない文字が多すぎる場合や、文字列をそのまま書いた方が便利な場合もあります。そんなときは、生の文字列リテラルが活躍します。

~~~rust
fn main() {

  let raw_str = r"Escapes don't work here: \x3F \u{211D}";

  println!("{}", raw_str);



  // 生の文字列に引用符が必要な場合は、#sのペアを追加します。

  let quotes = r#"And then I said: "There is no escape!""#;

  println!("{}", quotes);



  // 文字列の中に「#」が必要な場合は、デリミタに#を多めに入れます。

  // 使用できる#の数に制限はありません。

  let longer_delimiter = r###"A string with "# in it. And even "##!"###;

  println!("{}", longer_delimiter);

}

~~~

UTF-8ではない文字列が必要ですか？strとStringは有効なUTF-8でなければなりません）。あるいは、ほとんどがテキストであるバイト配列が欲しいのかもしれません。バイト文字列の出番です。

~~~rust
use std::str;

fn main() {

  // これは実際には `&str` ではないことに注意してください。

  let bytestring: &[u8; 21] = b"this is a byte string";



  // バイト配列には `Display` という特性がないので、printするには少し制限があります。

  println!("A byte string: {:?}", bytestring);



  // バイト文字列は、バイトエスケープを持つことができます...

  let escaped = b"\x52\x75\x73\x74 as bytes";

  // ...しかし、ユニコードエスケープはありません

  // `let escaped = b"\u{211D}`は許されない;

  println!("Some escaped bytes: {:?}", escaped);





  // 生のバイト文字列は、生の文字列と同様に動作します。

  let raw_bytestring = br"\u{211D} is not escaped here";

  println!("{:?}", raw_bytestring);



  // バイト配列の `str` への変換に失敗することがある

  if let Ok(my_str) = str::from_utf8(raw_bytestring) {

    println!("And the same as text: '{}'", my_str);

  }



  let _quotes = br#"You can also use "fancier" formatting, \

          like with normal raw strings"#;



  // バイト文字列はUTF-8である必要はありません。

  let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // "ようこそ" in SHIFT-JIS



  // しかし、その場合、常に `str` に変換できるとは限りません。

  match str::from_utf8(shift_jis) {

    Ok(my_str) => println!("Conversion successful: '{}'", my_str),

    Err(e) => println!("Conversion failed: {:?}", e),

  };

}

~~~

文字エンコーディング間の変換については、[encoding](https://crates.io/crates/encoding)クレートをご覧ください。

文字列リテラルやエスケープ文字の書き方については、『Rust Reference』の[Token](https://doc.rust-lang.org/reference/tokens.html)の章で詳しく説明されています。

---

### [?](https://doc.rust-jp.rs/rust-by-example-ja/std/result/question_mark.html)

幸いなことに、`?`演算子を使用することで、再びきれいな状態にすることができます。`?`は、`Result`を返す式の最後に使用されます。これは`match`式と同じで、`Err(err)`分岐は初期の`Err(From::from(err))`に展開され、`Ok(ok)`分岐は `ok`式に展開されます。

```rust
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
    
    // Intermediate function
    fn op_(x: f64, y: f64) -> MathResult {
        // if `div` "fails", then `DivisionByZero` will be `return`ed
        let ratio = div(x, y)?;

        // if `ln` "fails", then `NonPositiveLogarithm` will be `return`ed
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(match why {
                MathError::NonPositiveLogarithm
                    => "logarithm of non-positive number",
                MathError::DivisionByZero
                    => "division by zero",
                MathError::NegativeSquareRoot
                    => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
```

Resultをマッピング/合成する方法は数多くありますので、必ず[ドキュメント](https://doc.rust-lang.org/std/result/index.html)を確認してください。

