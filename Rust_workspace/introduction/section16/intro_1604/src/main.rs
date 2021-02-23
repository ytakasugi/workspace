struct Droppable {
    name: &'static str
}

// `Dropable`構造体に対して`Drop`トレイトを実装
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a"};

    // block A
    {
        let _b = Droppable { name: "b" };

        // block b
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // `drop`関数を用いて変数を手動で開放することもできます。
    drop(_a);

    println!("end of the main function");
    // `_a`はここで`drop`されることは *ない* 。なぜならば、上ですでに
    // （手動で）`drop`されているため。
}
