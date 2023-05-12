pub type Result<F, E = anyhow::Error> = anyhow::Result<F, E>;

pub fn init_env() {
    if let Err(e) = log4rs::init_file("log4rs.yml", Default::default()) {
        use env_logger::{Env, DEFAULT_FILTER_ENV};
        env_logger::init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));
        log::error!("Failed to init log4rs: {}", e);
    }
    if let Ok(v) = ini::Ini::load_from_file(".env") {
        if let Some(section) = v.section(None::<String>) {
            section.iter().for_each(|(k, v)| {
                std::env::set_var(k.to_uppercase(), v);
                log::info!("{k}={v}");
            });
        }
    }
}
