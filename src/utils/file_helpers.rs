use crate::kinesis::kinesis::{/*KinesisInput, */ KinesisRecord};
use serde_json;
use std::{
    error::Error,
    fs::{/*File, */ OpenOptions},
    io::{BufWriter /*, Read*/, Write},
};

// pub fn read_kinesis_json_file(file_path: &str) -> Result<KinesisInput, Box<dyn Error>> {
//     let mut file = File::open(file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//
//     let data: KinesisInput = serde_json::from_str(&contents)?;
//
//     Ok(data)
// }

pub fn write_output_to_file(record: &KinesisRecord) -> Result<(), Box<dyn Error>> {
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
// ******************************* tests ************************************ //
// ******************************* tests ************************************ //
// ******************************* tests ************************************ //
// ******************************* tests ************************************ //
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_read_kinesis_json_file() {
    //     let data = read_kinesis_json_file("./mock_data/input.json").unwrap();
    //     assert_eq!(data.records.len(), 10);
    // }

    #[test]
    #[ignore = "dont want to write to file during tests"]
    fn test_write_output_to_file() {
        let record = KinesisRecord::default_output();
        assert!(write_output_to_file(&record).is_ok());
    }
}
