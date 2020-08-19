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
//  #[serde(rename="dns_record")]  
  pub dns_records: Vec<DNSRecord>
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let mut s = Config::new();
    s.merge(File::with_name("test"))?;
    println!("{:#?}", s);
    s.merge(File::with_name("test1"))?;

    s.set("debug", false)?;

    println!("{:#?}", s);

    s.try_into()
  }
}  
