pub fn data_dir() -> String {
    match std::env::var("DATA_DIR") {
        Ok(data_dir) => data_dir,
        Err(_) => "./data".to_string(),
    }
}

pub fn saves_dir() -> String {
    format!("{}/save", data_dir())
}

pub fn exports_dir() -> String {
    format!("{}/export", data_dir())
}
