use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct BBResult {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}
