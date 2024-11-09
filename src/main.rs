mod kinesis;
mod mapper;
mod utils;
mod validator;

use kinesis::kinesis::{KinesisEvent, KinesisRecord};
use mapper::mapper::map_v2_data;
use utils::encoder::{decode_user_info, encode_data};
use utils::file_helpers::write_output_to_file;
use utils::http_helpers::{parse_event_body, parse_kinesis_event};
use validator::{types::V1UserInformation, validator::ModelValidator, validator::Validator};

use std::error::Error;

use lambda_http::{run, service_fn, tracing, Body, Request, Response};
use serde_json::Value;

use std::convert::Into;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

async fn function_handler(event: Request) -> Result<Response<Body>, lambda_http::Error> {
    let event_body = parse_event_body(event).unwrap_or(Value::Null);
    let kinesis_event = parse_kinesis_event(event_body).unwrap_or(KinesisEvent { records: vec![] });

    for mut record in kinesis_event.records {
        if let Err(e) = process_kinesis_events(&mut record) {
            eprintln!("Error processing record {:#?}: {:#?}", &record.event_id, e);
        }
        println!(
            "Processed kinesis record: {:?}\n\n",
            record.event_id.clone()
        );
    }

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from("Processed Kinesis records"))
        .map_err(Box::new)?;
    Ok(resp)
    // Ok(create_response(Resp::SUCCESS)?)
}

fn process_kinesis_events(record: &mut KinesisRecord) -> Result<(), Box<dyn Error>> {
    let user_info: V1UserInformation = decode_user_info(&record.kinesis.data)?;

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

    // not sure if this needs to be encoded or if kinesis does it for us
    let encoded_data = encode_data(&serde_json::to_string(&v2_data)?);

    record.kinesis.data = encoded_data;

    write_output_to_file(&record)?; // send back/write to db??
                                    // println!("Processed record ID {:?}:\n", user_info.id);

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
    fn test_process_kinesis_events() {
        let mut data = KinesisRecord::default();
        match process_kinesis_events(&mut data) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}

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
