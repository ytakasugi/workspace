### 標準ライブラリ

本テキストは、Rustを学習していく中で使用する標準ライブラリについて、記載していく。

詳細は、[標準ライブラリの公式ドキュメント](https://doc.rust-lang.org/stable/std/)を参照のこと。

- #### BufReader<R>

  - Description

    BufReader<R>構造体は、任意のReaderにバッファを追加する。

    Readインスタンスを直接操作するのは非常に非効率である。例えば、TCPStreamで読み取りを呼び出す度にシステムコールが発生します。BufReader<R>は、一度にある程度の量を読み取り、その結果をメモリ内のバッファに保持する。

    BufReader<R>が削除されると、そのバッファの内容が破棄する。

    同じストリーム上に複数のBufReader<R>のインスタンスを作成すると、データが損失する可能性がある。

    BufReader::into_innerでBufReaderをunwrapしたあと、基となるReaderから読み取りを行うと、データを損失することがある。

  - Example

    ~~~rust
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    
    fn main() -> std::io::Result<()> {
        let f = File::open("log.txt")?;
        let mut reader = BufReader::new(f);
    
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        println!("First line is {} bytes long", len);
        Ok(())
    }
    ~~~

  - new関連関数

    デフォルトのバッファ容量を持つ新しい BufReader<R> を作成します。デフォルトは現在 8 KB 。

- AsRef

  簡単な参照間変換を行う。

  このトレイトは、可変参照間の変換に使用される`FnMut`に似ている。

  もし、高度な変換を行う必要がある場合は、`From`を&T型で実装するか、カスタム関数を実装するほうがよい。

  AsRefは、参照と同じシグネチャを持っていますが、いくつか異なる点がある。

   - AsRefとは異なり、参照は任意のTに対してブランケット実装(トレイト境界を満たすあらゆる型にトレイトを実装すること)を持っており、参照または値のどちらかを受け取るために使用できる

   - 参照では、参照した値の[Hash]、[Eq]、[Ord]が同等であることが要求される

   - 構造体の単一フィールドのみを借用したい場合はAsrefを実施できますが、参照は実装できない。

     

     Note:このトレイトは失敗することができない。変換に失敗する可能性がある場合は、Option<T>またはResult<T, E>を返す専用のメソッドを使用すること。

 - Generic Implementations

   AsRef は、内部の型が参照または変異可能な参照である場合に自動参照を行う (例: foo.as_ref() は、foo が &mut Foo または &&mut Foo の型を持っていても同じように動作する)。

- Example

  ~~~rust
  fn is_hello<T: AsRef<str>>(s: T) {
     assert_eq!("hello", s.as_ref());
  }
  
  let s = "hello";
  is_hello(s);
  
  let s = "hello".to_string();
  is_hello(s);
  ~~~

- Path

  パスのスライス。

  この型はパスを検査するための多くの操作をサポートします。パスをその構成要素に分割したり(Unixでは/で区切って、Windowsでは/または/で区切って)、ファイル名を抽出したり、パスが絶対パスかどうかを判断したりなど。

  非サイズ型であり、常に 参照 や [Box] のようなポインタの後ろで使用されなければならない。

- File::open

  読み取り専用でファイルを開く。

  この関数は、パスが既に存在しない場合にエラーを返す。

- BufRead::lines

  Readerの行のイテレータを返す。

  この関数から返されるイテレータは、io::Result<[String]>のインスタンスを返します。返される各文字列は、最後に改行バイト（0xAバイト）やCRLF（0xD、0xAバイト）は持たない。

- std::env::args

  このプログラムが開始されたときの引数を返す（通常はコマンドライン経由で渡される）

- nth(n)

  イテレータの n 番目の要素を返す。

  ほとんどのインデックス操作と同様に、カウントはゼロから始まるので、 nth(0) は最初の値を返し、 nth(1) は 2 番目の値を返します。

  返された要素と同様に、先行するすべての要素がイテレータから消費されることに注意すること。つまり、先行する要素は破棄され、同じイテレータで nth(0) を複数回呼び出すと、異なる要素が返されることになる。

  nth() は、n がイテレータの長さ以上であれば [None] を返す。