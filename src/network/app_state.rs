use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};

use crate::network::profile::Profile;
use crate::network::transition::Transition;

#[derive(Clone)]
pub struct AppState {
    pub profiles: Arc<RwLock<HashMap<String, Profile>>>,
    pub transitions: Arc<Mutex<Vec<Transition>>>,
}
