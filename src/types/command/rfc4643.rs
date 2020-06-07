use std::fmt;

use super::NntpCommand;

/// Authenticate via `AUTHINFO` as specified in [RFC 4643](https://tools.ietf.org/html/rfc4643)
///
/// # Limitations
///
/// * SASL is not currently implemented
#[derive(Clone, Debug)]
pub enum AuthInfo {
    User(String),
    Pass(String),
}

impl fmt::Display for AuthInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthInfo::User(username) => write!(f, "AUTHINFO USER {}", username),
            AuthInfo::Pass(password) => write!(f, "AUTHINFO PASS {}", password),
        }
    }
}

impl NntpCommand for AuthInfo {}
