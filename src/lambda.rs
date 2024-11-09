mod kinesis;
mod mapper;
mod utils;
mod validator;

use mapper::mapper::map_v2_data;
use serde_json::from_slice;
use validator::{types::V1UserInformation, validator::ModelValidator, validator::Validator};
use utils::file_helpers::output;

use std::convert::Into;
use std::error::Error;

use aws_lambda_events::encodings::Base64Data;
use aws_lambda_events::event::kinesis::{KinesisEvent, KinesisEventRecord as KinesisRecord};
use lambda_runtime::{run, service_fn, tracing, LambdaEvent};
//
// cargo lambda invoke lambda --data-file input.json
// cargo lambda invoke lambda --data-file record.json
//
//
#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}

async fn function_handler(event: LambdaEvent<KinesisEvent>) -> Result<(), lambda_runtime::Error> {
    if event.payload.records.is_empty() {
        tracing::info!("No records in the event. Exiting.");
        return Ok(());
    }

    let mut records_copy: Vec<KinesisRecord> = event.payload.records.clone();
    for record in &mut records_copy {
        if let Err(e) = process_kinesis_events(record) {
            eprintln!("Error processing record {:#?}: {:#?}", &record.event_id, e);
        }
        println!("Processed kinesis record: {:?}\n", record.event_id.clone());
    }

    tracing::info!(
        "Successfully processed {} records.",
        event.payload.records.len()
    );
    Ok(())
}

fn process_kinesis_events(record: &mut KinesisRecord) -> Result<(), Box<dyn Error>> {
    let raw_data: &Vec<u8> = &record.kinesis.data.clone();
    let user_info: V1UserInformation = from_slice(&raw_data)?;

    if !ModelValidator.validate_v1(&user_info) {
        eprintln!("Record {:?} is invalid, dropping record\n\n", user_info.id);
        let error_message = format!("Invalid record: {:?}", user_info.id);
        return Err(error_message.into());
    } else {
        println!("V1 Record {:?} is valid", user_info.id);
    }

    let v2_data = map_v2_data(&user_info)?;
    if !ModelValidator.validate_v2(&v2_data) {
        eprintln!("Record {:?} is invalid, dropping record", user_info.id);
        let error_message = format!("Invalid record: {:?}", user_info.id);
        return Err(error_message.into());
    } else {
        println!(
            "V2 Record {:?} was transformed correctly and is valid",
            user_info.id
        );
    }

    let serialized_v2_data = serde_json::to_string(&v2_data)?;
    let encoded_v2_data = serialized_v2_data.into_bytes();
    let base64_encoded_v2_data = Base64Data(encoded_v2_data);

    println!("Base64 encoded V2 data: {:?}", record);

    record.kinesis.data = base64_encoded_v2_data;

    // write_output_to_file(&record)?; // send back/write to db??
    output(&record)?; // send back/write to db??

    Ok(())
}

