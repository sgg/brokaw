use std::io::{BufRead, BufReader, Read};

use flate2::bufread::ZlibDecoder;
use std::io;

/// A type of compression enabled on the server
#[derive(Copy, Clone, Debug)]
pub enum Compression {
    /// Giganews style compression
    XFeature,
}

/// An codec that can unpack compressed data streams
#[derive(Debug)]
pub(crate) enum Decoder<S> {
    XFeature(BufReader<ZlibDecoder<S>>),
    Passthrough(S),
}

impl Compression {
    pub(crate) fn use_decoder(&self, first_line: impl AsRef<[u8]>) -> bool {
        match self {
            Self::XFeature => first_line.as_ref().ends_with(b"[COMPRESS=GZIP]\r\n"),
        }
    }

    pub(crate) fn decoder<S: BufRead + Read>(&self, stream: S) -> Decoder<S> {
        match self {
            Self::XFeature => Decoder::XFeature(BufReader::new(ZlibDecoder::new(stream))),
        }
    }
}

impl<S: Read + BufRead> Read for Decoder<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Decoder::XFeature(d) => d.read(buf),
            Decoder::Passthrough(s) => s.read(buf),
        }
    }
}

impl<S: BufRead> BufRead for Decoder<S> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            Decoder::XFeature(d) => d.fill_buf(),
            Decoder::Passthrough(s) => s.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            Decoder::XFeature(d) => d.consume(amt),
            Decoder::Passthrough(s) => s.consume(amt),
        }
    }
}

/*
    In theory if we wanted to implement extensible compression we could replace Decoder and
    Compression objects w/ traits. That said it didn't seem necessary given the slow moving
    nature of the NNTP standard. If users ask for this we can always revisit it.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_decoder() {
        assert!(
            Compression::XFeature.use_decoder("224 xover information follows [COMPRESS=GZIP]\r\n")
        );
        assert!(!Compression::XFeature.use_decoder("224 xover information follows [COMPRESS=GZIP]"))
    }

    #[test]
    fn test_compressed() {
        let compressed_resp = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/xover_resp_xfeature_compress"
        ));
        let plain_resp = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/xover_resp_plain_text"
        ));

        let line_boundary = compressed_resp
            .iter()
            .enumerate()
            .find(|(_i, &byte)| byte == b'\n')
            .map(|(i, _)| i)
            .unwrap();

        let (first_line, data_blocks) = (
            &compressed_resp[..line_boundary + 1],
            &compressed_resp[line_boundary + 1..],
        );

        assert!(Compression::XFeature.use_decoder(first_line));

        let mut decoder = Compression::XFeature.decoder(&data_blocks[..]);
        let mut buf = String::new();
        decoder.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, String::from_utf8(plain_resp.to_vec()).unwrap())
    }
}
