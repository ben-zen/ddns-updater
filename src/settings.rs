use config::{ConfigError, Config, File};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DNSRecord {
  pub host: String,
  pub key: String
}

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub debug: bool,
  pub dns_record:  DNSRecord
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let mut s = Config::new();
    s.merge(File::with_name("test"))?;

    s.set("debug", false)?;

    s.try_into()
  }
}  
