use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoCreate {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoUpdate {
    pub completed: bool,
}
