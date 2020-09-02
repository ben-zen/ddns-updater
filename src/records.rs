use serde::{Deserialize};
use std::collections::HashMap;
use std::convert::From;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub enum RecordType {
  A,
  AAAA
}

impl Default for RecordType {
  fn default() -> Self {
    RecordType::A
  }
}

impl std::fmt::Display for RecordType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      RecordType::A => { "A" }
      RecordType::AAAA => { "AAAA" }
    })
  }
}

#[derive(Debug, Deserialize)]
pub struct DNSRecord {
  pub host: String,
  pub key: String,
  pub interface: String,
  #[serde(default)]
  pub record_type: RecordType
}

#[derive(Debug)]
pub enum Error {
  IoError(std::io::Error),
  PathError(String),
  TomlError(toml::de::Error)
}

// Implement the From trait for both toml::de::Error and std::io::Error
impl From<toml::de::Error> for Error {
  fn from(err : toml::de::Error) -> Self {
    Error::TomlError(err)
  }
}

impl From<std::io::Error> for Error {
  fn from(err : std::io::Error) -> Self {
    Error::IoError(err)
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    Some(self)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::IoError(err) => {
        write!(f, "{}", err)
      }
      Error::PathError(err) => {
        write!(f, "Unexpected path: {}", err)
      }
      Error::TomlError(err) => {
        write!(f, "{}", err)
      }
    }
  }
}

fn parse_slice(slice: &[u8]) -> Result<HashMap<String, DNSRecord>,
                                      Error> {
  match toml::from_slice(slice) {
    Ok(m) => { Ok(m) }
    Err(e) => { Err(Error::TomlError(e)) }
  }
}

pub fn parse(source_path : &Path) -> Result<HashMap<String, DNSRecord>,
                                            Error> {
  // If we the provided path is one file, only open that file.
  // Otherwise, find all .toml files in the directory
  if source_path.is_file() {
    if Some(OsStr::new("toml")) == source_path.extension() {
      let source_data = fs::read(source_path)?;
      parse_slice(&source_data)
    } else {
      // This will look a little messy, but paths aren't necessarily UTF-8, and
      // strings in Rust necessarily are. So we'll get a Cow<str> and then get a
      // String out of that.
      Err(Error::PathError(source_path.to_string_lossy().to_string()))
    }
  } else if source_path.is_dir() {
    // Get all toml files in the directory and parse them each as above.
    let mut records : HashMap<String, DNSRecord> = HashMap::new();
    for entry in fs::read_dir(source_path)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_file() {
        if Some(OsStr::new("toml")) == path.extension() {
          println!("Loading from {:?}", path);
          let data = std::fs::read(path)?;
          let file_records = parse_slice(&data)?;
          records.extend(file_records);
        }
      }
    }
    Ok(records)
  } else {
    panic!("Not file, not folder. Something's up with this path: {:?}",
           source_path);
  }
}  

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn parse_single_record() -> Result<(), Error> {
    let toml_string = r"
[demo]
host = 'demo.example.org'
key = 'demo-secret'
interface = 'enp3s0'
";
    let records = parse_slice(toml_string.as_bytes())?;
    assert_eq!(records.len(), 1); 
    assert_eq!(records.values().next().unwrap().host, "demo.example.org");
    Ok(())
  }

  #[test]
  fn fail_parsing_malformed_record() -> Result<(), Error> {
    let toml_string = r"
[demo]
host = 'demo.example.org'
";
    matches::assert_matches!(parse_slice(toml_string.as_bytes()),
                             Result::Err(Error::TomlError(_)));
    Ok(())
  }

  #[test]
  fn parse_ipv6_record() -> Result<(), Error> {
    let toml_string = r"
[demo]
host = 'demo.example.org'
key = 'demo-secret'
interface = 'enp3s0'
record_type = 'AAAA'
";
    let records = parse_slice(toml_string.as_bytes())?;
    matches::assert_matches!(records["demo"].record_type, RecordType::AAAA);
    Ok(())
  }

  #[test]
  fn parse_multiple_records() -> Result<(), Error> {
    let toml_string = r"
[demo]
host = 'demo1.example.org'
key = 'demo1-secret'
interface = 'enp3s0'

[other-demo]
host = 'demo2.example.org'
key = 'demo2-secret'
interface = 'enp3s0'
";
    let records = parse_slice(toml_string.as_bytes())?;
    assert_eq!(records.len(), 2);
    assert_eq!(records["demo"].host, "demo1.example.org");
    assert_eq!(records["other-demo"].host, "demo2.example.org");
    Ok(())
  }

  #[test]
  fn read_single_file() -> Result<(), Error> {
    let records = parse(Path::new("./data/test.toml"))?;
    assert_eq!(records.len(), 2);
    assert_eq!(records["demo-example-com"].host, "demo.example.com");
    assert_eq!(records["demo1-example-com"].host, "demo1.example.com");
    Ok(())
  }

  #[test]
  fn read_all_files_in_dir() -> Result<(), Error> {
    let records = parse(Path::new("./data"))?;
    assert_eq!(records.len(), 3);
    // I didn't want to check all the others.
    assert_eq!(records["demo2"].host, "demo2.example.com");
    Ok(())
  }
}
    
