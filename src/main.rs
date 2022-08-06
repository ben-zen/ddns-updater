mod records;
mod updater;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Config {
  #[structopt(parse(from_os_str), long)]
  source_path: PathBuf,
  #[structopt(default_value="https://ifconfig.me/ip", long)]
  ip_lookup: String,
  #[structopt(long, short)]
  what_if: bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
  let args = Config::from_args();
  let records = records::parse(&args.source_path)?;
  for record in records.values() {
    updater::update_ddns_record(&record,
                                &args.ip_lookup,
                                args.what_if).await?;
  }
  Ok(())
}
