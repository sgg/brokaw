use std::collections::HashMap;

/// The headers on an article
///
/// Note that per [RFC 5322](https://tools.ietf.org/html/rfc5322#section-3.6) headers
/// may be repeated
///
/// [Header Folding](https://tools.ietf.org/html/rfc3977#appendix-A.1)
#[derive(Clone, Debug)]
pub struct Headers(pub HashMap<String, Vec<String>>);

impl Headers {
    /// The total number of headers
    ///
    /// Note that this may be _more than_ the number of keys as headers may be repeated
    pub fn len(&self) -> usize {
        // FIXME(perf): simply pre-calculate this when we create Headers
        self.0.values().map(Vec::len).sum()
    }

    fn iter() {} // TODO(header iterator)
}
