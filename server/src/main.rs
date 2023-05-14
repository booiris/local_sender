fn main() {
    shared::init_env();
    log::info!("Server starting...");
    if let Err(e) = server::run() {
        eprintln!("Error: {}", e);
    }
}
