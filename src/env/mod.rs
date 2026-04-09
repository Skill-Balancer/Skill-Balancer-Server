pub fn data_dir() -> String {
    match std::env::var("DATA_DIR") {
        Ok(data_dir) => data_dir,
        Err(_) => "./data".to_string(),
    }
}

pub fn checkpoints_dir() -> String {
    format!("{}/checkpoints", data_dir())
}
pub fn print_steps() -> bool {
    match std::env::var("PRINT_STEPS") {
        Ok(value) => value == "true",
        Err(_) => false,
    }
}

pub fn exports_dir() -> String {
    format!("{}/export", data_dir())
}

pub fn db_url() -> String {
    match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => format!("sqlite://{}/local.db", data_dir()),
    }
}
