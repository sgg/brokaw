// TODO: Finish example

use std::path::PathBuf;
use std::fs::read_to_string;

use serde::Deserialize;
use structopt::StructOpt;
use log::*;
use brokaw::*;
use brokaw::types::command::Post;
use brokaw::types::prelude::RawResponse;

/// Post an article to a newsgroup
#[derive(Clone, Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    article: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct Article {
    body: String,
    from: String,
    headers: Vec<(String, String)>,
    message_id: Option<String>,
    newsgroups: Vec<String>,
    subject:  String
}

fn main() -> anyhow::Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let Opt {
        article
    } = &Opt::from_args();

    info!("Reading article from {}", article.display());

    let article: Article = {
        let json_string = read_to_string(article)?;
        println!("{}", json_string);
        serde_json::from_str(json_string.as_str())
            .map_err(|e| {
                error!("Failed to parse article from JSON file -- {}", e);
                e
            })?
    };
    info!("Creating the article...");
    println!("{:#?}", article);
    let (initiate, post) = Post::builder()
        .body(article.body)
        .headers(article.headers)
        .header("from", article.from)
        .header("Newsgroups", article.newsgroups.join(" "))
        .header("Subject", article.subject)
        .build();

    let mut client = ClientConfig::new()
        .authinfo_user_pass("newsreader", "readthenews")
        .connect(("localhost", 119))?;

    let initial_resp = client.command(initiate)?
        .fail_unless(340)?;

    info!("Server returned: {}", initial_resp.first_line_to_utf8_lossy());

    let body_resp = client.command(post)?
        .fail_unless(240)?;
    info!("Server returned: {}", body_resp.first_line_to_utf8_lossy());
    if let Some(db) = &body_resp.data_blocks() {
        info!("Response had datablocks {}", db.payload_as_utf8()?)
    }

    info!("Byeee 👋");

    Ok(())
}
