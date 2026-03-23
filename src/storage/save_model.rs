use burn::{
    module::Module,
    prelude::Backend,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
};
use std::env;

use crate::{models::ppo::Net, storage::utils::create_dir};

/// Load Burn model from disk and return the model
///  # Arguments
///  * `model` - The model to load the weights into (Must be the same architecture as the saved model)
///  * `name` - Filename without extension
///  * `device` - The device to load the model on
///
/// # Returns
/// The loaded model with the weights from disk
///
/// # Examples
/// ```rust
/// write me later
/// ```
pub fn load_model<B: Backend>(model: Net<B>, name: &String, device: &B::Device) -> Net<B> {
    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
    let path = format!("{}/{}", get_model_path(), name);
    let model = model
        .load_file(path, &recorder, device)
        .expect("Failed to load model");
    return model;
}

/// Save Burn model to disk and return the path
/// # Arguments
///
/// * `model` - The model to save
/// * `name` - Filename without extension
///
/// # Returns
/// The path to the saved model
///
/// # Examples
/// ```rust
/// write me later
/// ```
pub fn save_model<B: Backend>(model: Net<B>, name: &String) -> String {
    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
    let path = format!("{}/{}", get_model_path(), name);
    create_dir(&get_model_path());

    model
        .save_file(&path, &recorder)
        .expect("Failed to save model");
    return path;
}

pub fn get_model_path() -> String {
    match env::var("DATA_DIR") {
        Ok(data_dir) => data_dir + "/models",
        Err(_) => "./data/models".to_string(),
    }
}
