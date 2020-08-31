use reqwest::Client;
use super::records;

pub async fn update_ddns_record(record: &records::DNSRecord, what_if: bool)
                                -> Result<(), Box<dyn std::error::Error>> {
  if (what_if) {
    println!("Update {} record for {} via {}.", record.record_type, record.host, record.interface);
    Ok(())
  }

  // Find the interface for the record
  let interface = pnet::datalink::interfaces().iter()
    .filter(|x| x.name == record.interface).next()?;

  // Make sure the interface is usable 
  let 
  
  let res = client.post("https://dyn.dns.he.net/nic/update")
    .form(&[("hostname", hostname), ("password", key)])
    .send()
    .await?;

  println!("Request response: {:?}", res.status());

  let res_text = res.text().await?;
  println!("Request body: {:?}", res_text);
  Ok(())
}
