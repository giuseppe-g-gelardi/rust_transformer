use super::types::{Friends, V1UserInformation, V2UserInformation};

pub trait Validator {
    fn validate_v1(&self, data: &V1UserInformation) -> bool;
    fn validate_v2(&self, data: &V2UserInformation) -> bool;
}

pub struct ModelValidator;
impl Validator for ModelValidator {
    fn validate_v1(&self, data: &V1UserInformation) -> bool {
        if data.id.to_string().len() != 16 || data.id.to_string().chars().nth(0).unwrap() != '2' {
            // panic!("ID is empty or the incorrect format, dropping record");
            eprintln!("ID is empty or the incorrect format, dropping record");
            return false
        }
        if data.name.is_empty() {
            // panic!("Name is empty, dropping record");
            eprintln!("Name is empty, dropping record");
            return false
        }
        if data.email.is_empty() {
            // panic!("Email is empty, dropping record");
            eprintln!("Email is empty, dropping record");
            return false
        }
        true
    }

    fn validate_v2(&self, data: &V2UserInformation) -> bool {
        if data.id.to_string().len() != 16 || data.id.to_string().chars().nth(0).unwrap() != '2' {
            // panic!("ID is empty or the incorrect format, dropping record");
            eprintln!("ID is empty or the incorrect format, dropping record");
            return false
        }
        true
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

    // Tests for V1UserInformation
    #[test]
    fn test_validate_v1() {
        // validate the V1UserInformation struct against Correct data
        // -- SHOULD PASS --
        let data = V1UserInformation::default();

        let validator = ModelValidator;
        assert_eq!(validator.validate_v1(&data), true);
    }

    #[test]
    // #[should_panic(expected = "ID is empty or the incorrect format, dropping record")]
    fn test_validate_v1_id() {
        let mut data = V1UserInformation::default();
        data.id = 1234567890;
        let validator = ModelValidator;
        assert_eq!(validator.validate_v1(&data), false);
    }

    #[test]
    // #[should_panic(expected = "Name is empty, dropping record")]
    fn test_validate_v1_name() {
        let mut data = V1UserInformation::default();
        data.name = String::new();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v1(&data), false);
    }

    #[test]
    // #[should_panic(expected = "Email is empty, dropping record")]
    fn test_validate_v1_email() {
        let mut data = V1UserInformation::default();
        data.email = String::new();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v1(&data), false);
    }

    // Tests for V2UserInformation
    // soon....
}
