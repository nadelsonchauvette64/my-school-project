fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..10);
    println!("{}", random_number);
}
