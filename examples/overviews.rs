// FIXME(examples) fix this example

/*
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use anyhow::Result;
use brokaw::raw::connection::{NntpConnection, TlsConfig};
use brokaw::raw::response::RawResponse;
use brokaw::types::command::*;
use log::*;
use native_tls::TlsConnector;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short)]
    address: String,
    #[structopt(long, short, default_value = "563")]
    port: u16,
    #[structopt(long, short)]
    group: String,
    #[structopt(long, parse(from_os_str))]
    auth_file: PathBuf,
    #[structopt(long)]
    no_tls: bool,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Clone, Debug, StructOpt)]
enum Cmd {
    Xover {
        #[structopt(short, long)]
        low: u64,
        #[structopt(short, long)]
        high: u64,
        #[structopt(short, long, parse(from_os_str))]
        out: Option<PathBuf>,
    },
    Group,
}

fn read_auth_file(path: impl AsRef<Path>) -> Result<(String, String)> {
    let auth = read_to_string(path)?;
    let auth = auth.split(":").collect::<Vec<_>>();

    let username = auth[0];
    let password = auth[1];

    Ok((username.to_owned(), password.to_owned()))
}

fn run_cmd(
    conn: &mut NntpConnection,
    command: impl NntpCommand,
    print_resp: bool,
) -> Result<RawResponse> {
    warn!("Sending -- {}", command);
    let resp = conn.command(&command)?;

    if resp.code().is_error() || resp.code().is_failure() {
        panic!("Failure -- {:?}", resp.code())
    }

    if print_resp {
        info!("{}", resp.first_line_as_utf8()?);
    }
    Ok(resp)
}

fn login(conn: &mut NntpConnection, username: &str, password: &str) -> Result<()> {
    run_cmd(conn, AuthInfo::User(username.to_string()), true)?;
    run_cmd(conn, AuthInfo::Pass(password.to_string()), true)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let opt = Opt::from_args();
    let Opt {
        address,
        port,
        group,
        auth_file,
        no_tls,
        cmd,
    } = &opt;

    let tls_config = if !no_tls {
        debug!("Creating TLS configuration");
        Some(TlsConfig::new(address.clone(), TlsConnector::new()?))
    } else {
        warn!("TLS is disabled! Your creds will be sent in the clear!");
        None
    };

    debug!("Connecting to {} on port {}", address, port);
    let (mut conn, _) = NntpConnection::connect(
        (address.as_str(), *port),
        tls_config,
        Duration::from_secs(5).into(),
    )?;
    debug!("Connected!");

    let (username, password) = read_auth_file(&auth_file)?;

    login(&mut conn, &username, &password);

    match cmd.clone() {
        Cmd::Xover { low, high, out } => {
            run_cmd(&mut conn, Group(group.clone()), true);
            let _overview = run_cmd(&mut conn, XOver::Range { low, high }, false)?;
            info!("XOVER COMPLETE");
            if let Some(path) = out {
                info!("Writing overviews to file `{}`", path.display());
                unimplemented!()
            }
        }
        Cmd::Group => {
            run_cmd(&mut conn, Group(group.clone()), true);
        }
    }
    Ok(())
}
*/
fn main() {
    unimplemented!()
}
