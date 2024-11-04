use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct V1UserInformation {
    pub id: i64,
    pub is_active: bool,
    pub balance: String,
    pub picture: String,
    pub age: i32,
    pub eye_color: String,
    pub name: String,
    pub gender: String,
    pub company: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub about: String,
    pub registered: String,
    pub latitude: f64,
    pub longitude: f64,
    pub tags: Vec<String>,
    pub friends: Vec<Friends>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Friends {
    pub id: String,
    pub name: String,
}

