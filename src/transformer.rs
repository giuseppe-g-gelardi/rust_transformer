// mod kinesis;
mod mapper;
mod validator;

use mapper::mapper::map_v2_data;
use serde_json::from_slice;
use validator::{types::V1UserInformation, validator::ModelValidator, validator::Validator};

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use aws_lambda_events::encodings::Base64Data;
use aws_lambda_events::event::kinesis::{KinesisEvent, KinesisEventRecord};
use lambda_runtime::{run, service_fn, tracing, LambdaEvent};
use serde_json;

/*
    cargo lambda invoke transformer --data-file input.json
    cargo lambda invoke transformer --data-file record.json
    cargo lambda invoke transformer --data-file mock.json
*/

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

    tracing::debug!("DEBUGReceived event: {:#?}", event);

    let mut records: Vec<KinesisEventRecord> = event.payload.records.clone();
    for record in &mut records {
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

fn process_kinesis_events(record: &mut KinesisEventRecord) -> Result<(), Box<dyn Error>> {
    let raw_data: &Vec<u8> = &record.kinesis.data;
    let user_info: V1UserInformation = from_slice(&raw_data)?;
    let validator = ModelValidator;

    println!("Validating record: {:?}", &record);

    validate(
        &user_info,
        |data| validator.validate_v1(data),
        &user_info.id.to_string(),
        "V1",
    )?;

    let v2_data = map_v2_data(&user_info)?;
    validate(
        &v2_data,
        |data| validator.validate_v2(data),
        &user_info.id.to_string(),
        "V2",
    )?;

    // Serialize the V2 data and encode it in base64 -- needed?? or does Kinesis do it for us
    let serialized_v2_data = serde_json::to_string(&v2_data)?.into_bytes();
    let base64_encoded_v2_data = Base64Data(serialized_v2_data);
    record.kinesis.data = base64_encoded_v2_data;
    output(&record)?; // send back/write to db??
    Ok(())
}

fn validate<T>(
    data: &T,
    validator: impl Fn(&T) -> bool,
    data_id: &str,
    record_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match validator(data) {
        true => {
            println!("{} Record {:?} is valid", record_type, data_id);
            Ok(())
        }
        false => {
            eprintln!("{} Record {:?} is invalid, dropping.", record_type, data_id);
            Err("Record is invalid")?
        }
    }
}

pub fn output(record: &KinesisEventRecord) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open("./output.json")?;

    let mut writer = BufWriter::new(file);
    let record_json = serde_json::to_string(&record)?;

    writeln!(writer, "{}", record_json)?;
    writer.flush()?;

    Ok(())
}
