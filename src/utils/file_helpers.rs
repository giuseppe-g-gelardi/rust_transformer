// mod validator;
// mod mapper;

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Write},
    // thread::sleep,
    time::Duration,
};

use crate::validator::{
    types::{V1UserInformation, V2UserInformation},
    // validator::ModelValidator,
    // validator::Validator,
};

// use crate::mapper::mapper::map_v2_data;

pub fn read_json_file(file_path: &str) -> Result<Vec<V1UserInformation>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v1_data: Vec<V1UserInformation> = serde_json::from_str(&contents)?;

    Ok(v1_data)
}

pub fn write_to_file(record: &V2UserInformation) -> Result<(), Box<dyn Error>> {
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

pub fn parse_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <small|medium|large>", args[0]);
        std::process::exit(1);
    }
    let size = args[1].as_str();
    size.to_string()
}

pub fn dataset(size: &str) -> (String, Duration) {
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
