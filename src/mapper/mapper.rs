// use std::error::Error;

use super::super::validator::validator::{ModelValidator, Validator};
use crate::validator::types::{
    AccountInformation, Address, ContactInformation, UserInformation, V1UserInformation,
    V2UserInformation,
};

pub struct Mapper;

pub fn map_v2_data(data: &V1UserInformation) -> V2UserInformation {
    let user_name = parse_user_name(&data).unwrap();
    let address = parse_address(&data).unwrap();

    let v2_data = V2UserInformation {
        id: data.id,
        account_information: AccountInformation {
            is_active: data.is_active,
            registered: data.registered.to_string(),
            balance: data.balance.to_string(),
        },
        user_information: UserInformation {
            first_name: user_name.first_name,
            last_name: user_name.last_name,
            age: data.age,
            gender: data.gender.to_string(),
            eye_color: data.eye_color.to_string(),
            picture: data.picture.to_string(),
            company: data.company.to_string(),
        },
        contact_information: ContactInformation {
            email: data.email.to_string(),
            phone: data.phone.to_string(),
            address: Address {
                street: address.street,
                city: address.city,
                state: address.state,
                zip: address.zip,
            },
        },
        tags: data.tags.clone(),
        profile: data.about.to_string(),
    };

    let is_valid: bool = ModelValidator.validate_v2(&v2_data).try_into().unwrap();
    if !is_valid {
        eprintln!("V2 data is invalid");
    };

    v2_data
}

struct ParsedName {
    first_name: String,
    last_name: String,
}
fn parse_user_name(name: &V1UserInformation) -> Option<ParsedName> {
    let split_name: Vec<&str> = name.name.split_whitespace().collect();
    match split_name.len() {
        2 => Some(ParsedName {
            first_name: split_name[0].to_string(),
            last_name: split_name[1].to_string(),
        }),
        _ => {
            eprintln!("Name is not in the correct format");
            None
        }
    }
}

fn parse_address(address: &V1UserInformation) -> Option<Address> {
    let split_address: Vec<&str> = address.address.split(",").map(|s| s.trim()).collect();
    if split_address.len() == 4 {
        if let Ok(zip) = split_address[3].parse::<i32>() {
            return Some(Address {
                street: split_address[0].to_string(),
                city: split_address[1].to_string(),
                state: split_address[2].to_string(),
                zip,
            });
        } else {
            eprintln!("ZIP code is not a valid number");
        }
    } else {
        eprintln!("Address is not in the correct format");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_name() {
        let data = V1UserInformation::default();
        let parsed_name = parse_user_name(&data).unwrap();
        assert_eq!(parsed_name.first_name, "Tommie");
        assert_eq!(parsed_name.last_name, "Whitaker");
    }

    #[test]
    fn test_parse_address() {
        let data = V1UserInformation::default();

        let parsed_address = parse_address(&data).unwrap();
        assert_eq!(parsed_address.street, "251 Osborn Street");
        assert_eq!(parsed_address.city, "Aurora");
        assert_eq!(parsed_address.state, "Connecticut");
        assert_eq!(parsed_address.zip, 5047);
    }

    #[test]
    fn test_map_v2_data() {
        let data = V1UserInformation::default();
        let mapped_data = map_v2_data(&data);

        assert_eq!(mapped_data, V2UserInformation::default())
    }
}
