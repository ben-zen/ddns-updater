use serde::{Deserialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct DNSRecord {
  pub host: String,
  pub key: String
}

pub fn parse(source_path : &Path) -> Result<HashMap<String, DNSRecord>, toml::de::Error> {
  // If we the provided path is one file, only open that file.
  // Otherwise, find all .toml files in the directory
  if source_path.is_file() {
    let source_data = std::fs::read(source_path).unwrap();
    toml::from_slice(&source_data) as Result<HashMap<String, DNSRecord>, toml::de::Error>
  } else {
    // Get all toml files in the directory and parse them each as above.
    panic!("Unimplemented yet.");
  }
}  
