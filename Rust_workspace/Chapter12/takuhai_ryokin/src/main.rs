/*
use proconio::input;

struct CheckSize {
    height: i32,
    width: i32,
    depth: i32,
}

impl CheckSize {
    fn check(&self) -> bool {
        180 >= self.height + self.width + self.depth
    }
}

#[derive(Debug)]
struct Ryokin {
    height: i32,
    width: i32,
    depth: i32,
    weight: i32,
}

impl Ryokin {
    fn ryokin(&self) {
        let length = self.height + self.width + self.depth;
        let fee: i32;
        let weight = self.weight;

        if length <= 90 {
            if length <= 5 {
                fee = 500;
            } else if length <= 10 {
                fee = 1000;
            } else {
                fee = 1500;
            }
        } else {
            if weight <= 5 {
                fee = 1000;
            }else if weight <= 10 {
                fee = 2000;
            } else {
                fee = 3000;
            }
        }
    }
}
*/
fn main() {
    /*input!{
        height: i32,
        width: i32,
        depth: i32,
        weight: i32
    }

    let size = CheckSize{ height, width, depth };

    if size.check() {
        let charge = Ryokin{ height, width, depth, weight };
        println!("fee = {:?}", charge.ryokin());
    } else {
        println!("Size over");
    }*/
}