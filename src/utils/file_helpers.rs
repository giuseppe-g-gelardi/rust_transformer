use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Write},
    time::Duration,
};

use crate::validator::types::{V1UserInformation, V2UserInformation};

pub fn read_json_file(file_path: &str) -> Result<Vec<V1UserInformation>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v1_data: Vec<V1UserInformation> = serde_json::from_str(&contents)?;

    Ok(v1_data)
}

pub fn write_to_file(record: &V2UserInformation) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open("./mock_data/output.json")?;

    let mut writer = BufWriter::new(file);
    let record_json = serde_json::to_string(&record)?;

    writeln!(writer, "{}", record_json)?;
    writer.flush()?;

    Ok(())
}

// pub fn write_encoded_data_to_file(record: &str) -> Result<(), Box<dyn Error>> {
//     let file = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .write(true)
//         .open("./mock_data/output.json")?;
//
//     let mut writer = BufWriter::new(file);
//     let record_json = serde_json::to_string(&record)?;
//
//     writeln!(writer, "{}", record_json)?;
//     writer.flush()?;
//
//     Ok(())
// }
//
pub fn parse_args() -> String {
    parse_args_from(std::env::args().collect()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    })
}

fn parse_args_from(args: Vec<String>) -> Result<String, String> {
    if args.len() < 2 {
        return Err(format!("Usage: {} <small|medium|large>", args[0]));
    }
    let size = args[1].as_str();
    Ok(size.to_string())
}

pub fn dataset(size: &str) -> (String, Duration) {
    let small = String::from("./mock_data/small.json");
    let medium = String::from("./mock_data/medium.json");
    let large = String::from("./mock_data/large.json");
    let small_invalid = String::from("./mock_data/small_invalid.json");
    let medium_invalid = String::from("./mock_data/medium_invalid.json");
    let large_invalid = String::from("./mock_data/large_invalid.json");
    let kinesis_input = String::from("./mock_data/input.json");

    match size {
        "small" => (small, Duration::from_millis(500)),
        "medium" => (medium, Duration::from_millis(100)),
        "large" => (large, Duration::from_secs(10)),
        "small_invalid" => (small_invalid, Duration::from_millis(500)),
        "medium_invalid" => (medium_invalid, Duration::from_millis(100)),
        "large_invalid" => (large_invalid, Duration::from_secs(10)),
        "input" => (kinesis_input, Duration::from_millis(500)),
        _ => (small, Duration::from_millis(500)),
    }
}

// ******************************* tests ************************************ //
// ******************************* tests ************************************ //
// ******************************* tests ************************************ //
// ******************************* tests ************************************ //
#[cfg(test)]
mod tests {
    use super::*;

    // ******************** start parse args tests ************************** //
    #[test]
    fn test_parse_args_valid_small() {
        let args = vec!["program_name".to_string(), "small".to_string()];
        assert_eq!(parse_args_from(args).unwrap(), "small");
    }

    #[test]
    fn test_parse_args_valid_medium() {
        let args = vec!["program_name".to_string(), "medium".to_string()];
        assert_eq!(parse_args_from(args).unwrap(), "medium");
    }

    #[test]
    fn test_parse_args_valid_large() {
        let args = vec!["program_name".to_string(), "large".to_string()];
        assert_eq!(parse_args_from(args).unwrap(), "large");
    }

    #[test]
    fn test_parse_args_invalid_missing_argument() {
        let args = vec!["program_name".to_string()];
        assert!(parse_args_from(args).is_err());
    }

    #[test]
    fn test_parse_args_invalid_too_many_arguments() {
        let args = vec![
            "program_name".to_string(),
            "small".to_string(),
            "extra_arg".to_string(),
        ];
        assert_eq!(parse_args_from(args).unwrap(), "small");
    }
    // ********************** end parse args tests ************************** //

    // ******************** start dataset tests ***************************** //
    #[test]
    fn test_dataset() {
        let (path, interval) = dataset("small");
        assert_eq!(path, "./mock_data/small.json");
        assert_eq!(interval, Duration::from_millis(500));
    }

    #[test]
    fn test_read_json_file_small() {
        let data = read_json_file("./mock_data/small.json").unwrap();
        assert_eq!(data.len(), 10);
    }

    #[test]
    fn test_read_json_file_medium() {
        let data = read_json_file("./mock_data/medium.json").unwrap();
        assert_eq!(data.len(), 100);
    }

    #[test]
    fn test_read_json_file_large() {
        let data = read_json_file("./mock_data/large.json").unwrap();
        assert_eq!(data.len(), 1000);
    }
    // *********************** end dataset tests **************************** //

    #[test]
    #[ignore = "dont want to write to file during tests"]
    fn test_write_to_file() {
        let record = V2UserInformation::default();
        assert!(write_to_file(&record).is_ok());
    }
}
