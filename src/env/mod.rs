pub fn data_dir() -> String {
    get_env_var("DATA_DIR", "./data")
}

pub fn checkpoints_dir() -> String {
    format!("{}/checkpoints", data_dir())
}
pub fn print_steps() -> bool {
    get_env_var("PRINT_STEPS", "false") == "true"
}

pub fn print_training() -> bool {
    get_env_var("PRINT_TRAINING", "true") == "true"
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

fn get_env_var(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(value) => value,
        Err(_) => default.to_string(),
    }
}
