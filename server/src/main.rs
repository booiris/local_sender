fn main() {
    if let Err(e) = server::run() {
        eprintln!("Error: {}", e);
    }
}
