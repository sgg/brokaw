use std::fs::File;
use std::io::Write;

use brokaw::raw::connection::NntpConnection;
use brokaw::types::command as cmd;
use brokaw::types::response::BinaryArticle;
use log::*;
use std::convert::TryFrom;

fn main() -> anyhow::Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let (mut conn, resp) = NntpConnection::connect(("news.mozilla.org", 119), None, None)?;


    conn.command(&cmd::Group("mozilla.dev.platform".to_string()))?;

    let raw_article = conn.command(&cmd::Article::Number(47661))?;
    let article = BinaryArticle::try_from(&raw_article)?;

    println!("Raw Article: {}", article);

    println!("Headers\n{:#?}", article.headers());

    let text_article = article.to_text()?;

    text_article
        .lines()
        .for_each(|line| println!("{}", line));
    Ok(())
}
