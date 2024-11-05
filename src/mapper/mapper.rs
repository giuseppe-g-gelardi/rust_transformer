use super::super::validator::validator::{ModelValidator, Validator};
use crate::validator::types::{
    AccountInformation, Address, ContactInformation, UserInformation, V1UserInformation,
    V2UserInformation,
};

use std::error::Error;

pub fn map_v2_data(data: &V1UserInformation) -> Result<V2UserInformation, Box<dyn Error>> {
    let user_name = parse_user_name(&data)?;
    let address = parse_address(&data)?;

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

    if !ModelValidator.validate_v2(&v2_data) {
        eprintln!("V2 data is invalid");
        return Err("V2 data is invalid".into());
    }

    Ok(v2_data)
}

#[derive(Debug, PartialEq)]
struct ParsedName {
    first_name: String,
    last_name: String,
}

fn parse_user_name(name: &V1UserInformation) -> Result<ParsedName, Box<dyn Error>> {
    let split_name: Vec<&str> = name.name.split_whitespace().collect();
    match split_name.len() {
        2 => Ok(ParsedName {
            first_name: split_name[0].to_string(),
            last_name: split_name[1].to_string(),
        }),
        _ => Err("Name is not in the correct format".into()),
    }
}

fn parse_address(address: &V1UserInformation) -> Result<Address, Box<dyn Error>> {
    let split_address: Vec<&str> = address.address.split(",").map(|s| s.trim()).collect();
    if split_address.len() == 4 {
        if let Ok(zip) = split_address[3].parse::<i32>() {
            Ok(Address {
                street: split_address[0].to_string(),
                city: split_address[1].to_string(),
                state: split_address[2].to_string(),
                zip,
            })
        } else {
            Err("ZIP code is not a valid number".into())
        }
    } else {
        Err("Address is not in the correct format".into())
    }
}

// ********************************* tests ******************************** //
// ********************************* tests ******************************** //
// ********************************* tests ******************************** //
// ********************************* tests ******************************** //
// ********************************* tests ******************************** //
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
    fn test_parse_user_name_invalid() {
        let mut data = V1UserInformation::default();
        data.name = "Tommie".to_string();
        let parsed_name = parse_user_name(&data);

        assert!(parsed_name.is_err());
        assert_eq!(
            parsed_name.unwrap_err().to_string(),
            "Name is not in the correct format"
        );
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
    fn test_parse_address_invalid() {
        let mut data = V1UserInformation::default();
        data.address = "251 Osborn Street, Aurora, Connecticut".to_string();
        let parsed_address = parse_address(&data);

        assert!(parsed_address.is_err());
        assert_eq!(
            parsed_address.unwrap_err().to_string(),
            "Address is not in the correct format"
        );
    }

    #[test]
    fn test_map_v2_data() {
        let data = V1UserInformation::default();
        let mapped_data = map_v2_data(&data).expect("V2 data is invalid");
        assert_eq!(mapped_data, V2UserInformation::default())
    }
}
