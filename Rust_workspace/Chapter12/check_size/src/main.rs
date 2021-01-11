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

fn main() {
    input!{
        height: i32,
        width: i32,
        depth: i32,
    }

    let size = CheckSize{height, width, depth};
    println!("{}", size.check());
}