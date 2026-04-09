use burn::{
    module::Module,
    prelude::Backend,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
};
use burn_store::ModuleSnapshot;
use burn_store::SafetensorsStore;

use crate::{
    env::{checkpoints_dir, exports_dir},
    models::ppo::Net,
    storage::utils::create_dir,
};
pub struct CheckPoint {
    pub config_name: String,
    pub id: String,
}

impl CheckPoint {
    pub fn new(config_name: String, id: String) -> Self {
        Self { config_name, id }
    }

    pub fn to_url(&self) -> String {
        format!("/models/checkpoints/{}/{}.mpk", self.config_name, self.id)
    }
    pub fn to_export_url(&self) -> String {
        format!(
            "/models/export/{}/{}.safetensors",
            self.config_name, self.id
        )
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
        let path = format!("{}/{}/{}", checkpoints_dir(), &self.config_name, &self.id);
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
        let path = format!("{}/{}/{}", checkpoints_dir(), &self.config_name, &self.id);
        create_dir(&checkpoints_dir());

        model
            .save_file(&path, &recorder)
            .expect("Failed to save model");
        return path;
    }

    pub fn export<B: Backend>(&self, model: Net<B>, device: &B::Device) -> Result<String, String> {
        create_dir(&format!("{}/{}", &exports_dir(), &self.config_name));
        let model = self.load(model, device)?;
        let mut store = SafetensorsStore::from_file(format!(
            "{}/{}/{}.safetensors",
            exports_dir(),
            &self.config_name,
            &self.id
        ))
        .overwrite(true);

        let res = model.save_into(&mut store);
        match res {
            Ok(_) => Ok(format!(
                "{}/{}/{}.safetensors",
                &exports_dir(),
                &self.config_name,
                &self.id
            )),
            Err(e) => Err(format!("Failed to export model: {}", e)),
        }
    }
}

pub fn list_checkpoints(config_name: String) -> Vec<CheckPoint> {
    let mut checkpoints = Vec::new();
    if let Ok(entries) = std::fs::read_dir(format!("{}/{}", checkpoints_dir(), config_name)) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".mpk") {
                    let model_id = file_name.trim_end_matches(".mpk").to_string();
                    checkpoints.push(CheckPoint::new(config_name.clone(), model_id));
                }
            }
        }
    }
    checkpoints
}

pub fn list_exports(config_name: String) -> Vec<CheckPoint> {
    let mut exports = Vec::new();
    if let Ok(entries) = std::fs::read_dir(format!("{}/{}", exports_dir(), config_name)) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".safetensors") {
                    let model_id = file_name.trim_end_matches(".safetensors").to_string();
                    exports.push(CheckPoint::new(config_name.clone(), model_id));
                }
            }
        }
    }
    exports
}

pub fn delete_config_files(config_name: &String) {
    let checkpoint_dir = format!("{}/{}", checkpoints_dir(), config_name);
    let export_dir = format!("{}/{}", exports_dir(), config_name);
    for dir in [checkpoint_dir, export_dir] {
        if std::fs::exists(&dir).is_ok_and(|exists| exists)
            && std::fs::remove_dir_all(&dir).is_err()
        {
            eprintln!("Failed to delete file: {}", dir);
        }
    }
}
