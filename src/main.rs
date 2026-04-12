fn main() {
    println!("NATS Pub/Sub Demo");
    println!();
    println!("Terminal 1 - Consumer:");
    println!("  cargo run --bin consumer");
    println!();
    println!("Terminal 2 - Producer:");
    println!("  cargo run --bin producer <message>");
}
