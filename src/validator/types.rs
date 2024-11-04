use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Friends {
    pub id: String,
    pub name: String,
}

// ************************************************************************** //

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2UserInformation {
    pub id: i64,
    pub account_information: AccountInformation,
    pub user_information: UserInformation,
    pub contact_information: ContactInformation,
    pub tags: Vec<String>,
    pub profile: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountInformation {
    pub is_active: bool,
    pub registered: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserInformation {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub gender: String,
    pub eye_color: String,
    pub picture: String,
    pub company: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContactInformation {
    pub email: String,
    pub phone: String,
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: i32,
}

// ********************************* helpers ******************************** //
impl V1UserInformation {
    pub fn default() -> Self {
        V1UserInformation {
            id: 2377983216433421, // this actually fails when the id is not correct, so thats good
            is_active: false,
            balance: "$2,580.12".to_string(),
            picture: "http://placehold.it/32x32".to_string(),
            age: 38,
            eye_color: "green".to_string(),
            name: "Tommie Whitaker".to_string(),
            gender: "female".to_string(),
            company: "DEVILTOE".to_string(),
            email: "tommiewhitaker@deviltoe.com".to_string(),
            phone: "+1 (821) 523-2115".to_string(),
            address: "251 Osborn Street, Aurora, Connecticut, 5047".to_string(),
            about: "Officia veniam excepteur eiusmod id dolore velit laborum enim deserunt pariatur. Nulla labore est adipisicing in ea culpa magna nostrud. Laboris consequat officia eiusmod do quis do commodo culpa laborum. Et sunt in nostrud amet non laboris sit in tempor Lorem. Incididunt id non pariatur veniam. Sunt reprehenderit voluptate est fugiat ipsum excepteur laborum nostrud id anim tempor id in fugiat. Officia nulla minim enim dolore est ipsum consectetur.\r\n".to_string(),
            registered: "2021-07-24T03:19:44 +04:00".to_string(),
            latitude: 37.659874,
            longitude: -80.443451,
            tags: vec![
                "occaecat".to_string(),
                "dolor".to_string(),
                "aliqua".to_string(),
                "consequat".to_string(),
                "mollit".to_string(),
                "ut".to_string(),
                "adipisicing".to_string(),
            ],
            friends: vec![
                Friends {
                    id: "6f016d1f-ac7a-4907-a158-6629c415d480".to_string(),
                    name: "Mercedes Wiggins".to_string(),
                },
                Friends {
                    id: "258abaae-e2c1-4425-a713-33badc8961e2".to_string(),
                    name: "Blanche Vega".to_string(),
                },
                Friends {
                    id: "f482c1d0-0c07-48bf-96d4-c0c0cb88c15b".to_string(),
                    name: "Laverne Combs".to_string(),
                },
            ],
        }
    }
}

impl V2UserInformation {
    pub fn default() -> Self {
        V2UserInformation {
        id: 2377983216433421,
        account_information: AccountInformation {
            is_active: false,
            registered: "2021-07-24T03:19:44 +04:00".to_string(),
            balance: "$2,580.12".to_string(),
        },
        user_information: UserInformation {
            first_name: "Tommie".to_string(),
            last_name: "Whitaker".to_string(),
            age: 38,
            gender: "female".to_string(),
            eye_color: "green".to_string(),
            picture: "http://placehold.it/32x32".to_string(),
            company: "DEVILTOE".to_string(),
        },
        contact_information: ContactInformation {
            email: "tommiewhitaker@deviltoe.com".to_string(),
            phone: "+1 (821) 523-2115".to_string(),
            address: Address {
                street: "251 Osborn Street".to_string(),
                city: "Aurora".to_string(),
                state: "Connecticut".to_string(),
                zip: 5047,
            }
        },
        tags: vec![
            "occaecat".to_string(),
            "dolor".to_string(),
            "aliqua".to_string(),
            "consequat".to_string(),
            "mollit".to_string(),
            "ut".to_string(),
            "adipisicing".to_string(),
        ],
        profile: "Officia veniam excepteur eiusmod id dolore velit laborum enim deserunt pariatur. Nulla labore est adipisicing in ea culpa magna nostrud. Laboris consequat officia eiusmod do quis do commodo culpa laborum. Et sunt in nostrud amet non laboris sit in tempor Lorem. Incididunt id non pariatur veniam. Sunt reprehenderit voluptate est fugiat ipsum excepteur laborum nostrud id anim tempor id in fugiat. Officia nulla minim enim dolore est ipsum consectetur.\r\n".to_string(),
        }
    }
}
