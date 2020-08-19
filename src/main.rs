mod settings;
mod updater;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();

  let conf = settings::Settings::new()?;
  updater::update_ddns_record(&client,
                              &conf.dns_record.host,
                              &conf.dns_record.key).await?;
  
  Ok(())
}
