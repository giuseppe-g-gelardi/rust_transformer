use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct V2UserInformation {
    pub id: String,
    pub account_information: AccountInformation,
    pub user_information: UserInformation,
    pub contact_information: ContactInformation,
    pub tags: Vec<String>,
    pub profile: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInformation {
    pub is_active: bool,
    pub registered: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub gender: String,
    pub eye_color: String,
    pub picture: String,
    pub company: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    pub email: String,
    pub phone: String,
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: i32,
}

