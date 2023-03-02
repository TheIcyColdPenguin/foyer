#[derive(Debug, serde::Serialize)]
pub struct Photo {
    pub id: u64,
    pub ext: String,
    pub timestamp: String,
}
