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