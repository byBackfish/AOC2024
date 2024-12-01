mod p1;
mod p2;

fn main() {
    // check the args, if its p1 then run p1, if its p2 then run p2
    let input = std::env::args().nth(1).expect("no input");
    if input == "p1" {
        p1::p1();
    } else if input == "p2" {
        p2::p2();
    } else {
        println!("Invalid input");
    }
}
