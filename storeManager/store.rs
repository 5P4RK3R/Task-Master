use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;
// use model;
use crate::model::task::{User, Task};

// use crate::models::{User, Task};

pub struct AppState {
    pub users: Mutex<HashMap<Uuid, User>>,
    pub tasks: Mutex<HashMap<Uuid, Vec<Task>>>,
}

// pub mod store;
