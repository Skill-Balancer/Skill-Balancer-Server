use burn::{
    module::Module,
    prelude::Backend,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
};
use burn_store::ModuleSnapshot;
use burn_store::SafetensorsStore;

use crate::{
    env::{exports_dir, saves_dir},
    models::ppo::Net,
    storage::utils::create_dir,
};
pub struct CheckPoint {
    pub model_id: String,
}

impl CheckPoint {
    pub fn new(model_id: String) -> Self {
        Self { model_id }
    }

    pub fn to_url(&self) -> String {
        format!("/models/save/{}.mpk", self.model_id)
    }
    pub fn to_export_url(&self) -> String {
        format!("/models/export/{}.safetensors", self.model_id)
    }

    /// Load Burn model from disk and return the model
    ///  # Arguments
    ///  * `model` - The model to load the weights into (Must be the same architecture as the saved model)
    ///  * `device` - The device to load the model on
    ///
    /// # Returns
    /// The loaded model with the weights from disk
    ///
    /// # Examples
    /// ```rust
    /// write me later
    /// ```
    pub fn load<B: Backend>(&self, model: Net<B>, device: &B::Device) -> Result<Net<B>, String> {
        let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
        let path = format!("{}/{}", saves_dir(), &self.model_id);
        model
            .load_file(path, &recorder, device)
            .map_err(|e| format!("Failed to load model: {}", e))
    }
    /// Save Burn model to disk and return the path
    /// # Arguments
    ///
    /// * `model` - The model to save
    ///
    /// # Returns
    /// The path to the saved model
    ///
    /// # Examples
    /// ```rust
    /// write me later
    /// ```
    pub fn save<B: Backend>(&self, model: Net<B>) -> String {
        let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
        let path = format!("{}/{}", saves_dir(), &self.model_id);
        create_dir(&saves_dir());

        model
            .save_file(&path, &recorder)
            .expect("Failed to save model");
        return path;
    }

    pub fn export<B: Backend>(&self, model: Net<B>, device: &B::Device) -> Result<String, String> {
        create_dir(&exports_dir());
        let model = self.load(model, device)?;
        let mut store = SafetensorsStore::from_file(format!(
            "{}/{}.safetensors",
            exports_dir(),
            &self.model_id
        ))
        .overwrite(true);

        let res = model.save_into(&mut store);
        match res {
            Ok(_) => Ok(format!("/export/{}.safetensors", &self.model_id)),
            Err(e) => Err(format!("Failed to export model: {}", e)),
        }
    }
}

pub fn list_saves() -> Vec<CheckPoint> {
    let mut saves = Vec::new();
    if let Ok(entries) = std::fs::read_dir(saves_dir()) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".mpk") {
                    let model_id = file_name.trim_end_matches(".mpk").to_string();
                    saves.push(CheckPoint::new(model_id));
                }
            }
        }
    }
    saves
}

pub fn list_exports() -> Vec<CheckPoint> {
    let mut exports = Vec::new();
    if let Ok(entries) = std::fs::read_dir(exports_dir()) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".safetensors") {
                    let model_id = file_name.trim_end_matches(".safetensors").to_string();
                    exports.push(CheckPoint::new(model_id));
                }
            }
        }
    }
    exports
}
