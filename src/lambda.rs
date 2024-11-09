mod kinesis;
mod mapper;
mod utils;
mod validator;

// use kinesis::kinesis::{KinesisEvent, KinesisRecord};

use mapper::mapper::map_v2_data;
use serde_json::from_slice;
// use utils::encoder::{decode_user_info, encode_data};
// use utils::file_helpers::write_output_to_file;
use validator::{types::V1UserInformation, validator::ModelValidator, validator::Validator};

use std::convert::Into;
use std::error::Error;

use aws_lambda_events::encodings::Base64Data;
use lambda_runtime::{run, service_fn, tracing, LambdaEvent};
//
// TODO: utilize the aws_lambda_events types, update the function_handler signature
use aws_lambda_events::event::kinesis::{KinesisEvent, KinesisEventRecord as KinesisRecord};
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
    // let user_info: V1UserInformation = decode_user_info(&record.kinesis.data)?;
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

    // tracing::info!("User info: {:#?}", user_info);

    // // not sure if this needs to be encoded or if kinesis does it for us
    // let encoded_data = encode_data(&serde_json::to_string(&v2_data)?);
    //
    // record.kinesis.data = encoded_data;
    record.kinesis.data = base64_encoded_v2_data;
    //
    // write_output_to_file(&record)?; // send back/write to db??
    output(&record)?; // send back/write to db??
                      // println!("Processed record ID {:?}:\n", user_info.id);

    Ok(())
}

use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub fn output(record: &KinesisRecord) -> Result<(), Box<dyn Error>> {
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

// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_process_kinesis_events() {
//         let mut data = KinesisRecord::default();
//         match process_kinesis_events(&mut data) {
//             Ok(_) => (),
//             Err(e) => eprintln!("Error: {:?}", e),
//         }
//     }
// }

// // use tokio::test;
//
// #[tokio::test]
// async fn test_function_handler() {
//         let mock_event_body = json!({
//             "Records": [
//                 {
//                     "eventID": "1",
//                     "kinesis": {
//                         "data": "some-base64-encoded-string"
//                     }
//                 }
//             ]
//         });
//
//
//     // Step 2: Create a mock Request (simulating an HTTP request for Lambda)
//     let request = Request::default();
//
//
//
//     // Step 3: Call the handler function
//     let response = function_handler(request).await.unwrap();
//
//     // Step 4: Verify the response
//     assert_eq!(response.status(), 200);
//     assert_eq!(response.body(), &Body::from("Processed Kinesis records"));
// }
//
//
//

// enum Resp {
//     SUCCESS,
//     FAILURE,
// }
//
// impl Into<Response<Body>> for Resp {
//     fn into(self) -> Response<Body> {
//         match self {
//             Resp::SUCCESS => Response::builder()
//                 .status(200)
//                 .header("content-type", "text/plain")
//                 .body(Body::from("Success"))
//                 .unwrap(),
//             Resp::FAILURE => Response::builder()
//                 .status(500)
//                 .header("content-type", "text/plain")
//                 .body(Body::from("Failure"))
//                 .unwrap(),
//         }
//     }
// }
//
// fn create_response(resp: Resp) -> Result<Response<Body>, lambda_http::Error> {
//     Ok(resp.into())
// }
