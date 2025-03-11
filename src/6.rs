// Generating a random Rust code snippet using rand crate
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen();
    println!("The generated number is {}", x);
}
