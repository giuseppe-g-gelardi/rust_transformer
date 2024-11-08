use std::error::Error;

use lambda_http::Request;
use serde_json;
use serde_json::Value;

use crate::kinesis::kinesis::KinesisEvent;

pub fn parse_event_body(event: Request) -> Result<Value, Box<dyn Error>> {
    let body_bytes = event.body().as_ref();
    let event_body: Value = serde_json::from_slice(&body_bytes)?;

    Ok(event_body)
}

pub fn parse_kinesis_event(event_body: Value) -> Result<KinesisEvent, Box<dyn Error>> {
    let kinesis_event: KinesisEvent = serde_json::from_value(event_body)?;

    Ok(kinesis_event)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_event_body() {
        let event = Request::default();
        let event_body = parse_event_body(event).unwrap_or(Value::Null);
        assert_eq!(event_body, Value::Null);
    }

    #[test]
    fn test_parse_kinesis_event() {
        let event_body = Value::Null;
        let kinesis_event =
            parse_kinesis_event(event_body).unwrap_or(KinesisEvent { records: vec![] });
        assert_eq!(kinesis_event.records.len(), 0);
    }
}
