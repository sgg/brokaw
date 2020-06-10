//! This example demonstrates how to use the NntpConnection to retrieve an article
use std::convert::TryFrom;

use brokaw::raw::connection::NntpConnection;
use brokaw::types::command as cmd;
use brokaw::types::prelude::*;

fn main() -> anyhow::Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let (mut conn, _resp) = NntpConnection::connect(("news.mozilla.org", 119), None, None)?;

    let group_resp = conn.command(&cmd::Group("mozilla.dev.platform".to_string()))?;
    let group = Group::try_from(&group_resp)?;

    let raw_article = conn.command(&cmd::Article::Number(group.high))?;

    let article = BinaryArticle::try_from(&raw_article)?;

    println!("Article ID {}", article.message_id());
    println!("Article # {}", article.number());
    println!("Article has {} headers", article.headers().len());
    println!("Article body:\n");

    let text_article = article.to_text()?;
    text_article.lines().for_each(|line| println!("{}", line));
    Ok(())
}
