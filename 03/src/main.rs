mod util;
mod p1;
mod p2;

fn main() {
    let input = std::env::args().nth(1).expect("no input");
    if input == "p1" {
        p1::p1();
    } else if input == "p2" {
        p2::p2();
    } else {
        println!("Invalid input");
    }
}
