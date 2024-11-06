mod mapper;
mod utils;
mod validator;

// use base64::prelude::*;
// use std::{error::Error, thread::sleep, time::Duration};

use mapper::mapper::map_v2_data;
use utils::file_helpers::{
    dataset, parse_args, read_json_file, /*write_encoded_data_to_file, */ write_to_file,
};
use validator::{types::V1UserInformation, validator::ModelValidator, validator::Validator};

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Write},
    thread::sleep,
    time::Duration,
};

use base64::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

// fn kmain() -> Result<(), Box<dyn Error>> {
//     let data = read_kinesis_json_file("./encode/mock_data/input.json")?;
//     let k = simulate_kinesis_stream(data.records, Duration::from_secs(1));
//     println!("{:#?}", k);
//
//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    // let size = parse_args();
    // let (path, interval) = dataset(&size);
    // let data = read_json_file(&path)?;
    //
    // simulate_kinesis_stream(data, interval); // init mock kinesis stream
    println!("main");

    let kinesis_data = read_kinesis_json_file("./mock_data/input.json")?;
    simulate_kinesis_data_stream(kinesis_data.records, Duration::from_secs(1))?;

    Ok(())
}

fn encode_data(data: &str) -> String {
    let encoded = BASE64_STANDARD.encode(&data);
    encoded
}

fn decode_user_info(data: &str) -> Result<V1UserInformation, Box<dyn Error>> {
    let decoded = BASE64_STANDARD.decode(data)?;
    let decoded_string = String::from_utf8(decoded)?;

    let user_info: V1UserInformation = serde_json::from_str(&decoded_string)?;

    Ok(user_info)
}

fn read_kinesis_json_file(file_path: &str) -> Result<KinesisInput, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: KinesisInput = serde_json::from_str(&contents)?;

    Ok(data)
}

// fn simulate_kinesis_stream(records: Vec<V1UserInformation>, interval: Duration) {
//     for record in records {
//         if let Err(e) = simulate_lambda_execution(&record) {
//             // stream calls lambda
//             eprintln!("Error processing record {:#?}: {:#?}", record.id, e)
//         }
//
//         sleep(interval);
//     }
// }

fn simulate_kinesis_data_stream(
    records: Vec<KinesisRecord>,
    interval: Duration,
) -> Result<(), Box<dyn Error>> {
    for mut record in records {
        //
        // let decoded_user_info = decode_user_info(&record.kinesis.data)?;
        //
        // let v2_data = map_v2_data(&decoded_user_info)?;
        // let encoded_v2_data = encode_data(&serde_json::to_string(&v2_data)?);
        //
        // record.kinesis.data = encoded_v2_data;
        // println!("{:#?}", &record);
        //

        if let Err(e) = simulate_kinesis_lambda_trigger(&mut record) {
            eprintln!(
                "Error processing record {:#?}: {:#?}",
                record.kinesis.sequence_number, e
            )
        }

        sleep(interval);
    }

    Ok(())
}

fn simulate_kinesis_lambda_trigger(record: &mut KinesisRecord) -> Result<(), Box<dyn Error>> {
    let decoded_user_info = decode_user_info(&record.kinesis.data)?;

    if !ModelValidator.validate_v1(&decoded_user_info) {
        eprintln!(
            "Record {:?} is invalid, dropping record\n\n",
            decoded_user_info.id
        );
    }

    let v2_data = map_v2_data(&decoded_user_info)?;
    let encoded_v2_data = encode_data(&serde_json::to_string(&v2_data)?);

    record.kinesis.data = encoded_v2_data;

    match write_output_to_file(&record) {
        Ok(_) => {
            println!(
                "Record {:?} is valid\n{:?}\n\n",
                decoded_user_info.id, v2_data
            ); // {:#?} pretty print
            ()
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}

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

// fn simulate_lambda_execution(record: &V1UserInformation) -> Result<(), Box<dyn Error>> {
//     if !ModelValidator.validate_v1(record) {
//         eprintln!("Record {:?} is invalid, dropping record\n\n", record.id);
//     }
//
//     let v2_data = map_v2_data(&record)?;
//
//     match write_to_file(&v2_data) {
//         Ok(_) => {
//             println!("Record {:?} is valid\n{:?}\n\n", record.id, v2_data); // {:#?} pretty print
//             ()
//         }
//         Err(e) => eprintln!("Error: {:?}", e),
//     }
//
//     Ok(())

    // let serialized = serde_json::to_string(&v2_data)?;
    // let serialized = serde_json::to_string(&record)?;
    // let encoded_data = encode_data(&serialized);
    // match write_encoded_data_to_file(&encoded_data) {
    //     Ok(_) => (),
    //     Err(e) => eprintln!("Error: {:?}", e),
    // }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_simulate_lambda_execution() {
//         let data = V1UserInformation::default();
//         match simulate_lambda_execution(&data) {
//             Ok(_) => (),
//             Err(e) => eprintln!("Error: {:?}", e),
//         }
//     }
//
//     #[test]
//     fn test_simulate_kinesis_stream() {
//         let data = vec![V1UserInformation::default()];
//         let interval = Duration::from_millis(500);
//         simulate_kinesis_stream(data, interval);
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisRecord {
    pub kinesis: Kinesis,
    pub event_source: String,
    pub event_version: String,
    #[serde(rename = "eventID")]
    pub event_id: String,
    pub event_name: String,
    pub invoke_identity_arn: String,
    pub aws_region: String,
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kinesis {
    pub data: String,
    pub partition_key: String,
    pub kinesis_schema_version: String,
    pub sequence_number: String,
    pub approximate_arrival_timestamp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KinesisInput {
    pub records: Vec<KinesisRecord>,
}

// fn simulate_kinesis_data_stream(
//     records: Vec<KinesisRecord>,
//     interval: Duration,
// ) -> Result<(), Box<dyn Error>> {
//     for mut record in records {
//         let decoded_user_info = decode_user_info(&record.kinesis.data)?;
//         println!("{:?}", decoded_user_info);
//         // decoded_user_info seems to be the actual json object
//         /*
//            decoded_user_info is the actual json object.
//
//            we take that object, and map it to v2 from the main binary
//
//            then.... record.kinesis.data = serde_json::to_string(v2_data)?; -- something like this
//
//            then we write the record to a file
//         */
//
//         // this will be:
//         // record.kinesis.data = serde_json::to_string(v2_data)?;
//         record.kinesis.data = serde_json::to_string(&decoded_user_info)?;
//
//         sleep(interval);
//     }
//
//     Ok(())
// }
