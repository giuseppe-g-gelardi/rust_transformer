use std::{error::Error, fs::File, io::Read, thread::sleep, time::Duration};

use base64::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_kinesis_json_file("./encode/mock_data/input.json")?;
    let k = simulate_kinesis_stream(data.records, Duration::from_secs(1));
    println!("{:#?}", k);

    Ok(())
}

// fn encode_data(data: &str) -> String {
//     let encoded = BASE64_STANDARD.encode(data);
//     encoded
// } // use this to encode the v2 data to base64

fn decode_user_info(data: &str) -> Result<V1UserInformation, Box<dyn Error>> {
    let decoded = BASE64_STANDARD.decode(data)?;
    let decoded_string = String::from_utf8(decoded)?;

    let user_info: V1UserInformation = serde_json::from_str(&decoded_string)?;

    Ok(user_info)
}

fn simulate_kinesis_stream(
    records: Vec<KinesisRecord>,
    interval: Duration,
) -> Result<(), Box<dyn Error>> {
    for mut record in records {
        let decoded_user_info = decode_user_info(&record.kinesis.data)?;
        println!("{:?}", decoded_user_info);
        // decoded_user_info seems to be the actual json object
        //

        /*
           decoded_user_info is the actual json object.

           we take that object, and map it to v2 from the main binary

           then.... record.kinesis.data = serde_json::to_string(v2_data)?; -- something like this

           then we write the record to a file
        */

        // this will be:
        // record.kinesis.data = serde_json::to_string(v2_data)?;
        record.kinesis.data = serde_json::to_string(&decoded_user_info)?;

        sleep(interval);
    }

    Ok(())
}

pub fn read_kinesis_json_file(file_path: &str) -> Result<KinesisInput, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: KinesisInput = serde_json::from_str(&contents)?;

    Ok(data)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1UserInformation {
    pub id: i64,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub balance: String,
    pub picture: String,
    pub age: i32,
    #[serde(rename = "eyeColor")]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Friends {
    pub id: String,
    pub name: String,
}

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
