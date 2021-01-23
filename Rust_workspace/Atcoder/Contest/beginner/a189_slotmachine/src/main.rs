use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
        c: String
    }

    if a==b && a==c && b==c {
        println!("Won")
    } else {
        println!("Lost")
    }
}
