mod settings;
mod updater;

use std::env;

enum Action {
  NoOp, // -h results in NoOp
  Update,
  WhatIf // -whatif results in printing actions but not updating
}

enum Options {
  Folder(String),
  Verbose
}

fn read_arguments () -> (Action, Vec<Options>) {
  // do I do this lispy or not?

  let mut action = Action::Update;
  for arg in env::args() {
    if arg.eq("-h") {
      action = Action::NoOp;
    }
    if arg.eq("-whatif") {
      action = Action::WhatIf;
    }
  }
  (action, vec![])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let conf = settings::Settings::new()?;

  let (action, options) = read_arguments();
  // find a "what if" option and print the actions that would be taken

  match action {
    Action::NoOp => { Ok(()) }
    Action::Update => {
      let client = reqwest::Client::new();
      for dns_record in conf.dns_records {
        updater::update_ddns_record(&client,
                                    &dns_record.host,
                                    &dns_record.key).await?;
      }
      Ok(())
    }
    Action::WhatIf => {
      for dns_record in conf.dns_records {
        println!("Update A record for {:?}", dns_record.host);
      }
      Ok(())
    }
  }
}
