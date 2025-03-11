use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewPhysician {
    pub name: String,
    pub specialization: String,
}
