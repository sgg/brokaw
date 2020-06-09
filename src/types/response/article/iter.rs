/// An iterator over the lines of an Article body
///
/// Created by [`BinaryArticle::lines`] and [`Body::lines`]
#[derive(Clone, Debug)]
pub struct Lines<'a> {
    pub(crate) payload: &'a [u8],
    pub(crate) inner: std::slice::Iter<'a, (usize, usize)>,
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(start, end)| &self.payload[*start..*end])
    }
}

/// An iterator over the unterimnated lines of an Article body
///
/// Created by [`BinaryArticle::unterminated`] and [`Body::Lines`]
#[derive(Clone, Debug)]
pub struct Unterminated<'a> {
    pub(crate) inner: Lines<'a>,
}

impl<'a> Iterator for Unterminated<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|b| &b[..b.len() - 2])
    }
}
