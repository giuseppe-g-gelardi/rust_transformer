mod kinesis;
mod mapper;
mod utils;
mod validator;

use kinesis::kinesis::KinesisRecord;
use mapper::mapper::map_v2_data;
use utils::encoder::{decode_user_info, encode_data};
use utils::file_helpers::write_output_to_file;
use validator::{types::V1UserInformation, validator::ModelValidator, validator::Validator};

use std::error::Error;

use lambda_http::{run, service_fn, tracing, Body, Request, Response};
use serde::Deserialize;
use serde_json;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

#[derive(Deserialize, Debug)]
struct KinesisEvent {
    #[serde(rename = "Records")]
    records: Vec<KinesisRecord>,
}

async fn function_handler(event: Request) -> Result<Response<Body>, lambda_http::Error> {
    let event_body = match parse_event_body(event) {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Failed to parse event as JSON: {}", e);
            Value::Null
        }
    };

    let kinesis_event = match parse_kinesis_event(event_body) {
        Ok(event) => event,
        Err(e) => {
            eprintln!("Failed to parse as KinesisEvent structure: {}", e);
            KinesisEvent { records: vec![] }
        }
    };

    for mut record in kinesis_event.records {
        if let Err(e) = simulate_kinesis_lambda_trigger(&mut record) {
            eprintln!("Error processing record {:#?}: {:#?}", &record.event_id, e);
        }
        println!("Processed kinesis record: {:?}", record.event_id.clone());
    }

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from("Processed Kinesis records"))
        .map_err(Box::new)?;
    Ok(resp)
}

// ********************************** main ********************************* //
// ********************************** main ********************************* //
// ********************************** main ********************************* //
// ********************************** main ********************************* //
// ********************************** main ********************************* //
// ********************************** main ********************************* //
// ********************************** main ********************************* //
// ********************************** main ********************************* //
// ********************************** main ********************************* //
// ********************************** main ********************************* //
// ********************************** main ********************************* //

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

fn parse_event_body(event: Request) -> Result<Value, Box<dyn std::error::Error>> {
    let body_bytes = event.body().as_ref();
    let event_body: Value = serde_json::from_slice(&body_bytes)?;

    Ok(event_body)
}

fn parse_kinesis_event(event_body: Value) -> Result<KinesisEvent, Box<dyn std::error::Error>> {
    let kinesis_event: KinesisEvent = serde_json::from_value(event_body)?;

    Ok(kinesis_event)
}

// fn simulate_kinesis_data_stream(records: Vec<KinesisRecord>) -> Result<(), Box<dyn Error>> {
//     for mut record in records {
//         if let Err(e) = simulate_kinesis_lambda_trigger(&mut record) {
//             eprintln!("Error processing record {:#?}: {:#?}", &record.event_id, e);
//         }
//         sleep(Duration::from_millis(250));
//     }
//     Ok(())
// }

// fn s_main() -> Result<(), Box<dyn Error>> {
//     let kinesis_data = read_kinesis_json_file("./mock_data/input.json")?;
//     // simulate_kinesis_data_stream(kinesis_data.records)?;
//
//     if let Err(e) = simulate_kinesis_data_stream(kinesis_data.records) {
//         eprintln!("Error processing Kinesis data stream: {:#?}", e);
//     }
//
//     Ok(())
// }

// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
// ********************************** tests ********************************* //
#[cfg(test)]
mod tests {
    use super::*;
    //
    //     #[test]
    //     fn test_simulate_kinesis_data_stream() {
    //         let data = vec![KinesisRecord::default()];
    //         match simulate_kinesis_data_stream(data) {
    //             Ok(_) => (),
    //             Err(e) => eprintln!("Error: {:?}", e),
    //         }
    //     }
    //
    #[test]
    fn test_simulate_kinesis_lambda_trigger() {
        let mut data = KinesisRecord::default();
        match simulate_kinesis_lambda_trigger(&mut data) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}

// {
//     "Records": [
//         {
//         "kinesis": {
//             "data": "eyJpZCI6MjM3Nzk4.........",
//             "partitionKey": "partitionKey-1",
//             "sequenceNumber": "2000000000000001",
//             "approximateArrivalTimestamp": 1510261770.0,
//             "kinesisSchemaVersion": "1.0"
//         },
//         "eventSource": "aws:kinesis",
//         "eventVersion": "1.0",
//         "eventID": "shardId-000000000000:2000000000000001",
//         "eventName": "aws:kinesis:record",
//         "invokeIdentityArn": "arn:aws:iam::EXAMPLE",
//         "awsRegion": "us-east-1",
//         "eventSourceARN": "arn:aws:kinesis:EXAMPLE"
//         }
//     ]
// }

// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct KinesisRecord {
//     pub kinesis: Kinesis,
//     pub event_source: String,
//     pub event_version: String,
//     #[serde(rename = "eventID")]
//     pub event_id: String,
//     pub event_name: String,
//     pub invoke_identity_arn: String,
//     pub aws_region: String,
//     #[serde(rename = "eventSourceARN")]
//     pub event_source_arn: String,
// }
//
// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Kinesis {
//     pub data: String,
//     pub partition_key: String,
//     pub sequence_number: String,
//     pub approximate_arrival_timestamp: f64,
//     pub kinesis_schema_version: String,
// }
//
// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(rename = "Records")]
// pub struct KinesisEvent {
//     pub records: Vec<KinesisRecord>,
// }
//
//
//
//
// parse kinesis event
//
// serde_json::from_value(event_body).map_err(|e| {
//     eprintln!("Failed to parse as KinesisEvent structure: {}", e);
//     Box::new(e) as Box<dyn std::error::Error>
// })
