use std::convert::TryFrom;

use brokaw::raw::connection::NntpConnection;
use brokaw::types::command as cmd;
use brokaw::types::response::BinaryArticle;

fn main() -> anyhow::Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let (mut conn, _resp) = NntpConnection::connect(("news.mozilla.org", 119), None, None)?;

    conn.command(&cmd::Group("mozilla.dev.platform".to_string()))?;

    let raw_article = conn.command(&cmd::Article::Number(47661))?;
    let article = BinaryArticle::try_from(&raw_article)?;

    println!("Raw Article: {}", article);

    println!("Headers\n{:#?}", article.headers());

    let text_article = article.to_text()?;

    text_article.lines().for_each(|line| println!("{}", line));
    Ok(())
}
