use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};
use std::io;

fn server_start() -> io::Result<()> {
    // 172.30.217.78:8080をlistenし、acceptしつづけるリスナーを作成
    // `?`後置演算子を使ってエラー時には単に関数から抜ける
    // このリスナーはResultで包まれている
    let lis = TcpListener::bind("172.30.217.78:8080")?;
    // コネクションがあるたびに`stream`を取り出す
    for stream in lis.incoming() {
        // streamを包むResultを一旦剥がす
        // エラーが出てもループを継続するために`?`は使わない
        let mut stream = match stream {
            Ok(stream) => stream,
            // acceptでエラーが起きたらそれを通知して次のループへ
            Err(e) => {
                println!("An error occurred while accepting a connection: {}", e);
                continue;
            }
        };

        // IO処理はブロックするので別スレッドを立てる
        // そうすることでリクエストを処理しつつ新たなコネクションを受け付けられる
        // スレッドはspawnしたあと見捨てるので返り値のスレッドハンドルは無視
        let _ = thread::spawn(
            // spawnの引数にはクロージャを渡す
            // クロージャは `|引数| 本体` あるいは `|引数| -> 返り値の型 { 本体 }` で作る
            // 関数と違って`()`でなくとも型を推論できるなら引数の型や`-> 返り値の型`を省略できる
            // `move`はクロージャが捕捉した変数（今回は`stream`）の所有権をクロージャにムーブするためのキーワード
            move || -> io::Result<()> {
                loop {
                    // 1024バイトのバッファをスタック
                    let mut b = [0; 1024];
                    // バッファにクライアントからの入力を読み込み
                    let n = stream.read(&mut b)?;
                    // 読み込んだバイト数が0ならストリームの終了（スレッドから抜ける）
                    if n == 0 {
                        return Ok(());
                    // それ以外であれば読み込んだバッファの内容を書き戻す
                    } else {
                        stream.write(&b[0..n])?;
                    }
                }});
    }
    Ok(())
}


fn main() {
    match server_start() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}