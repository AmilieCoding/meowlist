use serde::{Deserialize, Serialize};

// -> Setting up task databases.
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }
}
