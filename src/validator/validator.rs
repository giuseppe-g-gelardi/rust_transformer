use super::types::{V1UserInformation, V2UserInformation};

pub trait Validator {
    fn validate_v1(&self, data: &V1UserInformation) -> bool;
    fn validate_v2(&self, data: &V2UserInformation) -> bool;
    fn valid_userid(&self, id: i64) -> bool;
}

pub struct ModelValidator;
impl Validator for ModelValidator {
    fn validate_v1(&self, data: &V1UserInformation) -> bool {
        // if data.id.to_string().len() != 16 || data.id.to_string().chars().nth(0).unwrap() != '2' {
        //     eprintln!("ID is empty or the incorrect format, dropping record");
        //     return false;
        // }
        if !self.valid_userid(data.id) {
            eprintln!("ID is empty or the incorrect format, dropping record");
            return false;
        }
        if data.name.is_empty() {
            eprintln!("Name is empty, dropping record");
            return false;
        }
        if data.email.is_empty() {
            eprintln!("Email is empty, dropping record");
            return false;
        }
        true
    }

    fn validate_v2(&self, data: &V2UserInformation) -> bool {
        // if data.id.to_string().len() != 16 || data.id.to_string().chars().nth(0).unwrap() != '2' {
        //     eprintln!("ID is empty or the incorrect format, dropping record");
        //     return false;
        // }
        if !self.valid_userid(data.id) {
            eprintln!("ID is empty or the incorrect format, dropping record");
            return false;
        }
        if data.account_information.registered.is_empty() {
            eprintln!("Registered is empty, dropping record");
            return false;
        }
        if data.user_information.first_name.is_empty() || data.user_information.last_name.is_empty()
        {
            eprintln!("First name and Last name are empty, dropping record");
            return false;
        }
        if data.contact_information.email.is_empty() {
            eprintln!("Email is empty, dropping record");
            return false;
        }
        true
    }

    fn valid_userid(&self, id: i64) -> bool {
        id.to_string().len() == 16 && id.to_string().chars().nth(0).unwrap() == '2'
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

    // **************************** test userid ***************************** //
    #[test]
    fn test_valid_userid() {
        let validator = ModelValidator;
        assert_eq!(validator.valid_userid(2377983216433421), true);
    }

    #[test]
    fn test_invalid_userid() {
        let validator = ModelValidator;
        assert_eq!(validator.valid_userid(1234567890), false);
    }

    // **************************** v1 Schema ******************************* //
    #[test]
    fn test_validate_v1() {
        let data = V1UserInformation::default();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v1(&data), true);
    }

    #[test]
    fn test_validate_v1_id() {
        let mut data = V1UserInformation::default();
        data.id = 1234567890;
        let validator = ModelValidator;
        assert_eq!(validator.validate_v1(&data), false);
    }

    #[test]
    fn test_validate_v1_name() {
        let mut data = V1UserInformation::default();
        data.name = String::new();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v1(&data), false);
    }

    #[test]
    fn test_validate_v1_email() {
        let mut data = V1UserInformation::default();
        data.email = String::new();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v1(&data), false);
    }

    // **************************** v2 Schema ******************************* //
    #[test]
    fn test_validate_v2() {
        let data = V2UserInformation::default();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v2(&data), true);
    }

    #[test]
    fn test_validate_v2_id() {
        let mut data = V2UserInformation::default();
        data.id = 1234567890;
        let validator = ModelValidator;
        assert_eq!(validator.validate_v2(&data), false);
    }

    #[test]
    fn test_validate_v2_registered() {
        let mut data = V2UserInformation::default();
        data.account_information.registered = String::new();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v2(&data), false);
    }

    #[test]
    fn test_validate_v2_firstname() {
        let mut data = V2UserInformation::default();
        data.user_information.first_name = String::new();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v2(&data), false);
    }

    #[test]
    fn test_validate_v2_lastname() {
        let mut data = V2UserInformation::default();
        data.user_information.last_name = String::new();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v2(&data), false);
    }

    #[test]
    fn test_validate_v2_email() {
        let mut data = V2UserInformation::default();
        data.contact_information.email = String::new();
        let validator = ModelValidator;
        assert_eq!(validator.validate_v2(&data), false);
    }
}
