use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::net::ToSocketAddrs;
use std::time::Duration;

use log::*;

use crate::error::{Error, Result};
use crate::raw::connection::{NntpConnection, TlsConfig};

use crate::raw::response::RawResponse;
use crate::types::command as cmd;
use crate::types::prelude::*;

/// A client
///
/// Each client represents a single connection
#[derive(Debug)]
pub struct NntpClient {
    conn: NntpConnection,
    config: ClientConfig,
    capabilities: Capabilities,
    group: Option<Group>,
}

impl NntpClient {
    /// Get the raw [`NntpConnection`] for the client
    ///
    /// # Usage
    ///
    /// NNTP is a **STATEFUL PROTOCOL** and misusing the underlying connection may mess up the
    /// state in the client that owns the connection.
    ///
    /// For example, manually sending a `GROUP`  command would leave change the group of
    /// the connection but will not update the NntpClient's internal record.
    ///
    /// Caveat emptor!
    pub fn conn(&mut self) -> &mut NntpConnection {
        &mut self.conn
    }

    /// Send a command
    ///
    /// This is useful if you want to use a command you have implemented or one that is not
    /// provided by a client method
    ///
    /// # Example
    ///
    /// Say we have a server that uses mode switching for whatever reason. Brokaw implements
    /// a [`ModeReader`](cmd::ModeReader) command but it does not provide a return type.
    /// We implement one in the following example
    /// <details><summary>MOTD</summary>
    ///
    /// ```no_run
    /// use std::convert::{TryFrom, TryInto};
    /// use brokaw::types::prelude::*;
    /// use brokaw::types::command as cmd;
    ///
    /// struct Motd {
    ///     posting_allowed: bool,
    ///     motd: String,
    /// }
    ///
    /// impl TryFrom<RawResponse> for Motd {
    ///     type Error = String;
    ///
    ///     fn try_from(resp: RawResponse) -> Result<Self, Self::Error> {
    ///         let posting_allowed = match resp.code() {
    ///             ResponseCode::Known(Kind::PostingAllowed) => true,
    ///             ResponseCode::Known(Kind::PostingNotPermitted) => false,
    ///             ResponseCode::Known(Kind::PermanentlyUnavailable) => {
    ///                 return Err("Server is gone forever".to_string());
    ///             }
    ///             ResponseCode::Known(Kind::TemporarilyUnavailable) => {
    ///                 return Err("Server is down?".to_string());
    ///             }
    ///             code => return Err(format!("Unexpected {:?}", code))
    ///         };
    ///         let mut motd = String::from_utf8_lossy(resp.first_line_without_code())
    ///             .to_string();
    ///
    ///         Ok(Motd { posting_allowed, motd })
    ///     }
    /// }
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use brokaw::client::{NntpClient, ClientConfig};
    ///     let mut client = ClientConfig::default()
    ///         .connect(("news.modeswitching.notreal", 119))?;
    ///
    ///     let resp: Motd = client.command(cmd::ModeReader, false)?.try_into()?;
    ///     println!("Motd: {}", resp.motd);
    ///     Ok(())
    /// }
    /// ```
    /// </details>
    pub fn command(&mut self, c: impl NntpCommand, force_multiline: bool) -> Result<RawResponse> {
        let resp = if force_multiline {
            self.conn.command_multiline(&c)
        } else {
            self.conn.command(&c)
        }?;

        Ok(resp)
    }

    /// Get the currently selected group
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Get the last selected group
    pub fn group(&self) -> Option<&Group> {
        self.group.as_ref()
    }

    /// Select a newsgroup
    pub fn select_group(&mut self, name: impl AsRef<str>) -> Result<Group> {
        let resp = self.conn.command(&cmd::Group(name.as_ref().to_string()))?;

        match resp.code() {
            ResponseCode::Known(Kind::GroupSelected) => {
                let group = Group::try_from(&resp)?;
                self.group = Some(group.clone());
                Ok(group)
            }
            ResponseCode::Known(Kind::NoSuchNewsgroup) => Err(Error::failure(resp)),
            code => Err(Error::Failure {
                code,
                msg: Some(format!("{}", resp.first_line_to_utf8_lossy())),
                resp,
            }),
        }
    }

    /// The capabilities cached in the client
    pub fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    /// Retrieve updated capabilities from the server
    pub fn update_capabilities(&mut self) -> Result<&Capabilities> {
        let resp = self
            .conn
            .command(&cmd::Capabilities)?
            .fail_unless(Kind::Capabilities)?;

        let capabilities = Capabilities::try_from(&resp)?;

        self.capabilities = capabilities;

        Ok(&self.capabilities)
    }

    /// Retrieve an article from the server
    ///
    ///
    /// # Text Articles
    ///
    /// Binary articles can be converted to text using the [`to_text`](BinaryArticle::to_text)
    /// and [`to_text_lossy`](BinaryArticle::to_text) methods. Note that the former is fallible
    /// as it will validate that the body of the article is UTF-8.
    ///
    /// ```
    /// use brokaw::client::NntpClient;
    /// use brokaw::error::Result;
    /// use brokaw::types::prelude::*;
    /// use brokaw::types::command::Article;
    ///
    /// fn checked_conversion(client: &mut NntpClient) -> Result<TextArticle> {
    ///     client.article(Article::Number(42))
    ///         .and_then(|b| b.to_text())
    /// }
    ///
    /// fn lossy_conversion(client: &mut NntpClient) -> Result<TextArticle> {
    ///     client.article(Article::Number(42))
    ///         .map(|b| b.to_text_lossy())
    /// }
    ///
    /// ```
    pub fn article(&mut self, article: cmd::Article) -> Result<BinaryArticle> {
        let resp = self.conn.command(&article)?.fail_unless(Kind::Article)?;

        resp.borrow().try_into()
    }

    /// Retrieve the body for an article
    pub fn body(&mut self, body: cmd::Body) -> Result<Body> {
        let resp = self.conn.command(&body)?.fail_unless(Kind::Head)?;
        resp.borrow().try_into()
    }

    /// Retrieve the headers for an article
    pub fn head(&mut self, head: cmd::Head) -> Result<Head> {
        let resp = self.conn.command(&head)?.fail_unless(Kind::Head)?;
        resp.borrow().try_into()
    }

    /// Retrieve the status of an article
    pub fn stat(&mut self, stat: cmd::Stat) -> Result<Option<Stat>> {
        let resp = self.conn.command(&stat)?;
        match resp.code() {
            ResponseCode::Known(Kind::ArticleExists) => resp.borrow().try_into().map(Some),
            ResponseCode::Known(Kind::NoArticleWithMessageId)
            | ResponseCode::Known(Kind::InvalidCurrentArticleNumber)
            | ResponseCode::Known(Kind::NoArticleWithNumber) => Ok(None),
            _ => Err(Error::failure(resp)),
        }
    }

    /// Close the connection to the server
    pub fn close(&mut self) -> Result<RawResponse> {
        let resp = self
            .conn
            .command(&cmd::Quit)?
            .fail_unless(Kind::ConnectionClosing)?;

        Ok(resp)
    }
}

/// Configuration for an [`NntpClient`]
#[derive(Clone, Debug)]
pub struct ClientConfig {
    tls_config: Option<TlsConfig>,
    authinfo: Option<(String, String)>,
    group: Option<String>,
    read_timeout: Option<Duration>,
}

impl ClientConfig {
    /// Perform an AUTHINFO USER/PASS authentication after connecting to the server
    ///
    /// https://tools.ietf.org/html/rfc4643#section-2.3
    pub fn authinfo_user_pass(&mut self, username: String, password: String) -> &mut Self {
        self.authinfo = Some((username, password));
        self
    }

    /// Change the tls configuration
    pub fn tls_config(&mut self, config: TlsConfig) -> &mut Self {
        self.tls_config = Some(config);
        self
    }

    /// Use the default TLS configuration
    pub fn default_tls(&mut self, domain: String) -> Result<&mut Self> {
        self.tls_config = Some(TlsConfig::default_connector(domain)?);
        Ok(self)
    }

    /// Join a group upon connection
    ///
    /// If this is set to None then no `GROUP` command will be sent when the client is initialized
    pub fn group(&mut self, name: Option<impl AsRef<str>>) -> &mut Self {
        self.group = name.map(|s| s.as_ref().to_string());
        self
    }

    /// The read timeout of the underlying socket
    pub fn read_timeout(&mut self, duration: Option<Duration>) -> &mut Self {
        self.read_timeout = duration;
        self
    }

    // FIXME(ux): Add better timeout support

    /// Resolves the configuration into a client
    pub fn connect(&self, addr: impl ToSocketAddrs) -> Result<NntpClient> {
        let (mut conn, conn_response) =
            NntpConnection::connect(addr, self.tls_config.clone(), self.read_timeout)?;

        debug!(
            "Connected. Server returned `{}`",
            conn_response.first_line_to_utf8_lossy()
        );

        // FIXME(ux) check capabilities before attempting auth info
        if let Some((username, password)) = &self.authinfo {
            if self.tls_config.is_none() {
                warn!("TLS is not enabled, credentials will be sent in the clear!");
            }
            debug!("Authenticating with AUTHINFO USER/PASS");
            authenticate(&mut conn, username, password)?;
        }

        debug!("Retrieving capabilities...");
        let capabilities = get_capabilities(&mut conn)?;

        let group = if let Some(name) = &self.group {
            debug!("Connecting to group {}...", name);
            select_group(&mut conn, name)?.into()
        } else {
            debug!("No initial group specified");
            None
        };

        Ok(NntpClient {
            conn,
            config: self.clone(),
            capabilities,
            group,
        })
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            tls_config: None,
            authinfo: None,
            group: None,
            read_timeout: None,
        }
    }
}

impl RawResponse {}

/// Perform an AUTHINFO USER/PASS exchange
fn authenticate(
    conn: &mut NntpConnection,
    username: impl AsRef<str>,
    password: impl AsRef<str>,
) -> Result<()> {
    debug!("Sending AUTHINFO USER");
    let user_resp = conn.command(&cmd::AuthInfo::User(username.as_ref().to_string()))?;

    if user_resp.code != ResponseCode::from(381) {
        return Err(Error::Failure {
            code: user_resp.code,
            resp: user_resp,
            msg: Some("AUTHINFO USER failed".to_string()),
        });
    }

    debug!("Sending AUTHINFO PASS");
    let pass_resp = conn.command(&cmd::AuthInfo::Pass(password.as_ref().to_string()))?;

    if pass_resp.code() != ResponseCode::Known(Kind::AuthenticationAccepted) {
        return Err(Error::Failure {
            code: pass_resp.code,
            resp: pass_resp,
            msg: Some("AUTHINFO PASS failed".to_string()),
        });
    }
    debug!("Successfully authenticated");

    Ok(())
}

fn get_capabilities(conn: &mut NntpConnection) -> Result<Capabilities> {
    let resp = conn.command(&cmd::Capabilities)?;

    if resp.code() != ResponseCode::Known(Kind::Capabilities) {
        Err(Error::failure(resp))
    } else {
        Capabilities::try_from(&resp)
    }
}

fn select_group(conn: &mut NntpConnection, group: impl AsRef<str>) -> Result<Group> {
    let resp = conn.command(&cmd::Group(group.as_ref().to_string()))?;

    match resp.code() {
        ResponseCode::Known(Kind::GroupSelected) => Group::try_from(&resp),
        ResponseCode::Known(Kind::NoSuchNewsgroup) => Err(Error::failure(resp)),
        code => Err(Error::Failure {
            code,
            msg: Some(format!("{}", resp.first_line_to_utf8_lossy())),
            resp,
        }),
    }
}
