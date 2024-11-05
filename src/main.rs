mod mapper;
mod utils;
mod validator;

use std::{error::Error, thread::sleep, time::Duration};

use mapper::mapper::map_v2_data;
use utils::file_helpers::{dataset, parse_args, read_json_file, write_to_file};
use validator::{types::V1UserInformation, validator::ModelValidator, validator::Validator};

fn main() -> Result<(), Box<dyn Error>> {
    let size = parse_args();

    let (path, interval) = dataset(&size);

    let data = read_json_file(&path)?;
    simulate_kinesis_stream(data, interval);

    Ok(())
}

fn simulate_kinesis_stream(records: Vec<V1UserInformation>, interval: Duration) {
    for record in records {
        if let Err(e) = simulate_lambda_execution(&record) {
            eprintln!("Error processing record {:#?}: {:#?}", record.id, e)
        }

        sleep(interval);
    }
}

fn simulate_lambda_execution(record: &V1UserInformation) -> Result<(), Box<dyn Error>> {
    if !ModelValidator.validate_v1(record) {
        eprintln!("Record {:?} is invalid, dropping record\n\n", record.id);
    }

    let v2_data = map_v2_data(&record)?;

    match write_to_file(&v2_data) {
        Ok(_) => {
            println!("Record {:?} is valid\n{:?}\n\n", record.id, v2_data); // {:#?} pretty print
            ()
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_lambda_execution() {
        let data = V1UserInformation::default();
        match simulate_lambda_execution(&data) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_simulate_kinesis_stream() {
        let data = vec![V1UserInformation::default()];
        let interval = Duration::from_millis(500);
        simulate_kinesis_stream(data, interval);
    }
}
