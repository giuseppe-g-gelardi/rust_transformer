use serde_json;
use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use aws_lambda_events::event::kinesis::KinesisEventRecord;

pub fn output(record: &KinesisEventRecord) -> Result<(), Box<dyn Error>> {
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
