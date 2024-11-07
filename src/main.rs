mod kinesis;
mod mapper;
mod utils;
mod validator;

use kinesis::kinesis::KinesisRecord;
use mapper::mapper::map_v2_data;
use utils::encoder::{decode_user_info, encode_data};
use utils::file_helpers::{read_kinesis_json_file, write_output_to_file};
use validator::{types::V1UserInformation, validator::ModelValidator, validator::Validator};

use std::{error::Error, thread::sleep, time::Duration};

use serde_json;

// TODO: build more mock data for testing -- with invalid data

fn simulate_kinesis_lambda_trigger(record: &mut KinesisRecord) -> Result<(), Box<dyn Error>> {
    let user_info: V1UserInformation = decode_user_info(&record.kinesis.data)?;

    if !ModelValidator.validate_v1(&user_info) {
        eprintln!("Record {:?} is invalid, dropping record\n\n", user_info.id);
        return Err("Invalid record".into());
    }

    let v2_data = map_v2_data(&user_info)?;
    let encoded_data = encode_data(&serde_json::to_string(&v2_data)?);

    record.kinesis.data = encoded_data;

    write_output_to_file(&record)?;
    println!("Processed record ID {:?}:\n", user_info.id);

    Ok(())
}

fn simulate_kinesis_data_stream(records: Vec<KinesisRecord>) -> Result<(), Box<dyn Error>> {
    for mut record in records {
        if let Err(e) = simulate_kinesis_lambda_trigger(&mut record) {
            eprintln!("Error processing record {:#?}: {:#?}", &record.event_id, e);
        }
        sleep(Duration::from_millis(250));
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let kinesis_data = read_kinesis_json_file("./mock_data/input.json")?;
    // simulate_kinesis_data_stream(kinesis_data.records)?;

    if let Err(e) = simulate_kinesis_data_stream(kinesis_data.records) {
        eprintln!("Error processing Kinesis data stream: {:#?}", e);
    }

    Ok(())
}

// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_kinesis_data_stream() {
        let data = vec![KinesisRecord::default()];
        match simulate_kinesis_data_stream(data) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_simulate_kinesis_lambda_trigger() {
        let mut data = KinesisRecord::default();
        match simulate_kinesis_lambda_trigger(&mut data) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
