use std::fmt;

use crate::types::NntpCommand;

/// Enable Giganews style header compression
#[derive(Clone, Copy, Debug)]
pub struct XFeatureCompress;

impl fmt::Display for XFeatureCompress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XFEATURE COMPRESS GZIP TERMINATOR")
    }
}

impl NntpCommand for XFeatureCompress {}
