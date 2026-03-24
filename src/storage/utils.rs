pub fn create_dir(path: &String) {
    std::fs::create_dir_all(path).expect("Failed to create directory");
}
