use std::thread;
use std::time::Duration;

fn main() {
    // thread::spawn関数を呼び出し、新規にスレッドを生成し実行したいコードを含むクロージャを渡している
    // 引数なしのクロージャは、forループで生成した値を返却し、thread::spawn関数はクロージャから返却された値を引数として実行している
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // メインスレッド
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
