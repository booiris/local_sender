use std::path::Path;

pub fn check_valid_path(path: &Path) -> Result<bool, String> {
    let base = std::env::current_dir()
        .map_err(|e| "[ls] can not get base dir. err: ".to_owned() + &e.to_string())?;

    Ok(path.starts_with(base))
}
