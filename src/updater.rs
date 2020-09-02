use super::records;

fn get_ip_for_interface_by_record_type(
  record_type: &records::RecordType,
  interface: &pnet::datalink::NetworkInterface) -> ipnetwork::IpNetwork {
  match record_type {
    records::RecordType::A => {
      let address = interface.ips.iter().filter(|x| { x.is_ipv4() }).next();
      match address {
        Some(x) => { x.clone() }
        None => { panic!("Supplied interface {} doesn't have an IPv4 address as required.", interface.name) }
      }
    }
    records::RecordType::AAAA => {
      let address = interface.ips.iter().filter(|x| { x.is_ipv6() }).next();
      match address {
        Some(x) => { x.clone() }
        None => { panic!("Supplied interface {} doesn't have an IPv6 address as required.", interface.name) }
      }
    }
  }
}

pub async fn update_ddns_record(record: &records::DNSRecord, what_if: bool)
                                -> Result<(), Box<dyn std::error::Error>> {
  if what_if {
    println!("Update {} record for {} via {}.", record.record_type, record.host, record.interface);
    Ok(())
  } else {

    let interfaces = pnet::datalink::interfaces();
    // Find the interface for the record
    let interface = match interfaces.iter()
      .filter(|x| x.name == record.interface).next() {
        Some(iface) => { iface }
        None => { panic!("Couldn't find expected interface {}.",
                         record.interface) }
      };

    // Make sure the interface is usable 
    let address = get_ip_for_interface_by_record_type(&record.record_type,
                                                      interface);
  
    // Create the client for this interface.
    let client = reqwest::Client::builder()
      .local_address(address.ip())
      .build()?;

    let res = client.post("https://dyn.dns.he.net/nic/update")
      .form(&[("hostname", &record.host), ("password", &record.key)])
      .send()
      .await?;

    println!("Request response: {:?}", res.status());

    let res_text = res.text().await?;
    println!("Request body: {:?}", res_text);
    Ok(())
  }
}
