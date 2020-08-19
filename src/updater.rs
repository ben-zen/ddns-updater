use reqwest::Client;

pub async fn update_ddns_record(client: &Client, hostname: &str, key: &str)
                                -> Result<(), Box<dyn std::error::Error>> {
  let res = client.post("https://dyn.dns.he.net/nic/update")
    .form(&[("hostname", hostname), ("password", key)])
    .send()
    .await?;

  println!("Request response: {:?}", res.status());

  let res_text = res.text().await?;
  println!("Request body: {:?}", res_text);
  Ok(())
}
