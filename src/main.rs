mod records;
mod updater;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Config {
  #[structopt(default_value="test.toml", parse(from_os_str), long)]
  source_path: PathBuf,
  #[structopt(long, short)]
  what_if: bool,
  #[structopt(long, short)]
  verbose: bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
  let args = Config::from_args();
  let records = records::parse(&args.source_path).unwrap();
  if args.what_if {
    for (_, dns_record) in records {
      println!("Update A record for {:?}", dns_record.host);
    }
  } else {
    let client = reqwest::Client::new();
    for (_, dns_record) in records {
      updater::update_ddns_record(&client,
                                  &dns_record.host,
                                  &dns_record.key).await?;
    }
  }
  Ok(())
}
