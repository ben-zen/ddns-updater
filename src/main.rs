mod updater;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  updater::update_ddns_record(&client, "host", "key").await?;
  
  Ok(())
}
