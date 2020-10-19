use std::time::Duration;

use log::*;

use brokaw::client::ClientConfig;
use brokaw::ConnectionConfig;
use structopt::StructOpt;

/// Connect to a server and get the info for a specified group
///
/// This example utilizes the high-level client API
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short)]
    address: String,
    #[structopt(long, short, default_value = "563")]
    port: u16,
    #[structopt(long, short)]
    group: String,
    #[structopt(long)]
    no_tls: bool,
    #[structopt(long, short)]
    username: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let Opt {
        address,
        port,
        group,
        no_tls: _,
        username,
    } = Opt::from_args();

    let password = rpassword::prompt_password_stderr("Password: ")?;

    info!("Creating config...");

    let config = {
        let mut config = ClientConfig::new();

        config
            .authinfo_user_pass(username, password)
            .group(Some(group))
            .connection_config(
                ConnectionConfig::default()
                    .read_timeout(Some(Duration::from_secs(5)))
                    .default_tls(&address)?
                    .to_owned(),
            );

        config
    };

    info!("Connecting...");
    let mut client = config.connect((address.as_str(), port))?;

    info!("Connected!");
    info!("Capabilities: {:#?}", client.capabilities());

    info!("Group info: {:?}", client.group());

    info!("Closing connection...");
    client.close()?;
    info!("Closed connection!");

    Ok(())
}
