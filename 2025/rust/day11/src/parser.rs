use crate::DeviceNetwork;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<DeviceNetwork> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut network = DeviceNetwork::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed.split(':').collect();
        if parts.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid line format: {}", trimmed),
            ));
        }

        let device = parts[0].trim().to_string();
        let outputs: Vec<String> = parts[1].split_whitespace().map(|o| o.to_string()).collect();

        network.add_device(device, outputs);
    }

    Ok(network)
}
