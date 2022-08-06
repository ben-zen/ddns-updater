use super::records;
use dnsclient::r#async::{DNSClient};
use dnsclient::UpstreamServer;
use std::net::{IpAddr};

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

pub async fn update_ddns_record(record: &records::DNSRecord, ip_lookup_addr: &str, what_if: bool)
                                -> Result<(), Box<dyn std::error::Error>> {

    let interfaces = pnet::datalink::interfaces();
    // Find the interface for the record
    let interface = match interfaces.iter()
      .filter(|x| x.name == record.interface).next() {
        Some(iface) => { iface }
        None => { panic!("Couldn't find expected interface {}.",
                         record.interface) }
      };
    
    let dns_client = DNSClient::new([UpstreamServer::new(([8, 8, 8, 8], 53))].to_vec());

    // Let's check if there is a record for the address.
    let current_addresses_opt = match dns_client.query_addrs(&record.host).await {
      Ok(x) => { Some(x) }
      Err(x) => {
        // record the failure
        println!("Error retrieving current DNS records: {}", x);
        None
      }
    };

    // Make sure the interface is usable 
    let address = get_ip_for_interface_by_record_type(&record.record_type,
                                                      interface);
  
    // Create the client for this interface.
    let client = reqwest::Client::builder()
      .local_address(address.ip())
      .build()?;
    
    // We want to check what our current public IP address is, too.
    println!("Querying {} for this machine's current public address.", ip_lookup_addr);
    let ip_lookup_result = client.get(ip_lookup_addr).send().await?;
    let ip_lookup = match ip_lookup_result.status().is_success() {
      true => match ip_lookup_result.text().await?.parse::<IpAddr>() {
        Ok(x) => { Some(x) }
        Err(x) => {
          println!("Error parsing the address: {}", x);
          None
        }
      }
      false => {
        println!("Didn't get an address back! Got status {}", ip_lookup_result.status());
        Option::<IpAddr>::None
      }
    };

    let mut run_update = true;
    
    // Now we want to check, do we have an address already presented for this domain?
    if let Some(current_ip) = ip_lookup {
      println!("Got our current address, checking for a match: {}", current_ip);
      if let Some(addresses) = current_addresses_opt {
        for address in addresses {
          println!("Checking {} against current public address {}", address, current_ip);
          if address == current_ip {
            println!("Found the current address, not updating.");
            run_update = false;
            break;
          }
        }
      }
    }

    if what_if && run_update {
      println!("If run, update the {} record for {}", record.record_type, record.host);
    } else if run_update {
      let res = client.post("https://dyn.dns.he.net/nic/update")
        .form(&[("hostname", &record.host), ("password", &record.key)])
        .send()
        .await?;

      println!("Request response: {:?}", res.status());

      let res_text = res.text().await?;
      println!("Request body: {:?}", res_text);
    }
    Ok(())
}
