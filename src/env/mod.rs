pub fn data_dir() -> String {
    match std::env::var("DATA_DIR") {
        Ok(data_dir) => data_dir,
        Err(_) => "./data".to_string(),
    }
}

pub fn models_dir() -> String {
    format!("{}/models", data_dir())
}
