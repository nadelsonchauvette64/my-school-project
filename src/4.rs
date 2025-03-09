// This is an example of Rust code that returns a random number between 0 and 100.
fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..=100);
    println!("{}", random_number);
}
