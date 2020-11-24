//use std::io;

fn main() {

    //let mut height = String::new();
    //let mut width = String::new();
    //let mut depth = String::new();

    //io::stdin().read_line(&mut height).expect("Faild to read line");
    //io::stdin().read_line(&mut width).expect("Faild to read line");
    //io::stdin().read_line(&mut depth).expect("Faild to read line");
    
    //let size = checksize {height(&height), width, depth}

    let size = Checksize {height: 50, width: 60, depth: 90};
    println!("message {}", size.check(&size));


    
    struct Checksize {
        height: u32,
        width: u32,
        depth: u32,
    }

    impl Checksize {
        fn check(&self, other: &Checksize) -> bool {
            180 >= other.height + other.width + other.depth
        }
    }
}
