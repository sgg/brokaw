use brokaw::types::command::{XFeatureCompress, XOver};

use brokaw::*;
use log::*;
use structopt::StructOpt;

/// A program for getting compressed headers
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short)]
    address: String,
    #[structopt(long, short, default_value = "563")]
    port: u16,
    /// The group to read the headers from
    #[structopt(long, short)]
    group: String,
    /// The number of headers to retrieve
    #[structopt(long, short)]
    num_headers: u32,
    #[structopt(long)]
    username: String,
    #[structopt(long)]
    password: Option<String>,
}

fn main() -> anyhow::Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let Opt {
        address,
        port,
        group,
        num_headers,
        username,
        password,
    } = Opt::from_args();

    let password = if let Some(pw) = password {
        pw
    } else {
        rpassword::prompt_password_stderr("Password: ")?
    };

    info!("Creating client...");

    let mut client = ClientConfig::default()
        .group(Some(group.clone()))
        .authinfo_user_pass(username, password)
        .connection_config(
            ConnectionConfig::new()
                .compression(Some(Compression::XFeature))
                .default_tls(&address)?
                .to_owned(),
        )
        .connect((address.as_str(), port))?;

    let group = client.group().unwrap().to_owned();

    info!(
        "Group {name} has a {number} headers ranging from {low} to {high}",
        name = group.name,
        low = group.low,
        high = group.high,
        number = group.number
    );

    info!("Enabling header compression");
    client.command(XFeatureCompress)?.fail_unless(290)?;

    let high = group.high;
    let low = high - num_headers;
    info!("Retrieving headers {} through {}", low, high);
    let resp = client.conn().command(&XOver::Range { low, high })?;
    resp.data_blocks().unwrap().lines().for_each(|header| {
        let s = String::from_utf8_lossy(header).to_string();
        println!("{}", s);
    });

    Ok(())
}
