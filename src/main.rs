mod mapper;
mod validator;

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Write},
    time::Duration,
};

use validator::{
    types::{V1UserInformation, V2UserInformation},
    validator::ModelValidator,
    validator::Validator,
};

use mapper::mapper::map_v2_data;

fn main() {
    let size = parse_args();

    let (path, interval) = dataset(&size);

    match read_json_file(&path) {
        Ok(data) => simulate_kinesis_stream(data, interval),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn simulate_kinesis_stream(records: Vec<V1UserInformation>, interval: Duration) {
    for record in records {
        let is_valid = ModelValidator.validate_v1(&record);
        if !is_valid {
            eprintln!("Record {:?} is invalid, dropping record\n\n", record.id);
            continue;
        }
        let v2_data = map_v2_data(&record);
        match write_to_file(&v2_data) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {:?}", e),
        }

        println!("Record {:?} is valid: ", record.id);
        println!("{:?}\n\n", v2_data);

        std::thread::sleep(interval);
    }
}
// this should return a result, nothing for success and an io error for failure
fn write_to_file(record: &V2UserInformation) -> Result<(), Box<dyn Error>> {
    // let file = File::create("output.txt")?;
    // serde_json::to_writer(file, &record)?;
    // Ok(())
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

fn parse_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <small|medium|large>", args[0]);
        std::process::exit(1);
    }
    let size = args[1].as_str();
    size.to_string()
}

fn read_json_file(file_path: &str) -> Result<Vec<V1UserInformation>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v1_data: Vec<V1UserInformation> = serde_json::from_str(&contents)?;

    Ok(v1_data)
}

fn dataset(size: &str) -> (String, Duration) {
    let small = String::from("./mock_data/small.json");
    let medium = String::from("./mock_data/medium.json");
    let large = String::from("./mock_data/large.json");

    match size {
        "small" => (small, Duration::from_millis(500)),
        "medium" => (medium, Duration::from_millis(100)),
        "large" => (large, Duration::from_secs(10)),
        _ => (small, Duration::from_millis(500)),
    }
}
