use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Collection {
    pub collection_id: String,
    pub name: String,
    pub thumbnail_image: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Collections {
    pub count: u16,
    pub results: Vec<Collection>,
    pub next: String,
}