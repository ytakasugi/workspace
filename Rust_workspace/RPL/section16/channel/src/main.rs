use std::thread;
use std::sync::mpsc;

fn main() {
    // mpsc::channel関数はタプルを返し、1つ目の要素は、送信側、2つ目の要素は受信側になる
    let (tx, rx) = mpsc::channel();

    // thread::spawnを使用して新しいスレッドを生成し、それからmoveを使用して、 立ち上げたスレッドがtxを所有するようにクロージャにtxをムーブしている
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let receive = rx.recv().unwrap();
    println!("Got: {}", receive);
}
