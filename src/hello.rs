use lambda_http::{run, service_fn, tracing, Body, Request, Response};

use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

async fn function_handler(event: Request) -> Result<Response<Body>, lambda_http::Error> {
    let event_body = parse_event_body(event).unwrap_or(Value::Null);
    println!("event_body: {:?}", event_body);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from("Hello, World!"))
        .map_err(Box::new)?;
    Ok(resp)
}

fn parse_event_body(event: Request) -> Result<Value, Box<dyn std::error::Error>> {
    let body_bytes = event.body().as_ref();
    let event_body: Value = serde_json::from_slice(&body_bytes)?;

    Ok(event_body)
}
