#[derive(Debug, serde::Serialize)]
pub struct Photo {
    pub id: u64,
    pub ext: String,
    pub timestamp: String,
}

#[derive(Debug, serde::Serialize)]
pub struct Tag {
    pub id: u64,
    pub name: String,
    pub colour: String,
}
