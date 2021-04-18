struct Charge {
    num: i32,
}

impl Charge {
    fn child_charge(&self) -> i32 {
        400 * self.num
    }

    fn adult_charge(&self) -> i32 {
        1200 * self.num
    }
}

fn main() {
    // 大人料金
    let num = 2;
    let charge = Charge{num};

    println!("{}", charge.adult_charge());

    // 子供料金
    let num = 3;
    let charge = Charge{num};

    println!("{}", charge.child_charge());

}