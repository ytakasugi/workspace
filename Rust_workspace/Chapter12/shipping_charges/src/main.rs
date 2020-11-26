use std::io;

/*struct Isok {
    height: u32,
    width: u32,
    depth: u32,
}

impl Isok {
    fn checksize (height: u32, width: u32, depth: u32) -> bool {
        return 180 >= height + width + depth
    }
}

struct Ryokin {
    height: u32,
    width: u32,
    depth: u32,
    weight: u32,
}

impl Ryokin {
    fn calc(&self) -> u32 {
        let  length = self.height + self.width + self.depth;
        let  ryokin: u32;
        let weight = self.weight;

        if length <= 90 {
            if weight <= 5 {
                ryokin = 500;
            } else if weight <= 10 {
                ryokin = 1000;
            } else {
                ryokin = 1500;
            }
        } else {
            if weight <= 5 {
                ryokin = 1000;
            }else if weight <= 10 {
                ryokin = 2000;
            } else {
                ryokin = 3000;
            }
        }
        return ryokin
    }
}
*/
fn main() {
    // 長さ入力する
    println!("What is the vartical length?:");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Faild to read line");
    // 幅を入力する
    println!("What is the width?:");
    let mut width = String::new();
    io::stdin().read_line(&mut width).expect("Faild to read line");
    // 重さを入力する
    println!("What is the depth?:");
    let mut depth = String::new();
    io::stdin().read_line(&mut depth).expect("Faild to read line");
}
