// material_lib/src/lib.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    pub model: String,
    pub brand: String,
    pub package: String,
    pub spec: String,
    pub code: String,
    pub quantity: u32,
}
