use std::sync::Mutex;

fn main() {
    // Mutex<T>はスマートポインタ。より正確を期すなら、lockの呼び出しがMutexGuardというスマートポインタを返却する
    // Mutex<T>を生成
    let m = Mutex::new(5);

    {
        // Mutex内部のデータにアクセスするには、 lockメソッドを使用してロックを獲得する
        // この呼び出しは、現在のスレッドをブロックする
        // 戻り値を中に入っているデータへの可変参照として扱うことができます。 
        // 型システムにより、mの値を使用する前にロックを獲得していることが確認される
        // Mutex<i32>はi32ではないので、 i32を使用できるようにするには、ロックを獲得しなければならない
        // 型システムにより、それ以外の場合に内部のi32にアクセスすることは許されない
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}