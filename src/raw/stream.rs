use std::net::TcpStream;

use native_tls::TlsStream;
use std::io;
use std::io::{Read, Write};

/// A raw NNTP session
#[derive(Debug)]
pub enum NntpStream {
    /// A stream using TLS
    Tls(TlsStream<TcpStream>),
    /// A plain text stream
    Tcp(TcpStream),
}

impl From<TlsStream<TcpStream>> for NntpStream {
    fn from(stream: TlsStream<TcpStream>) -> Self {
        Self::Tls(stream)
    }
}

impl From<TcpStream> for NntpStream {
    fn from(stream: TcpStream) -> NntpStream {
        Self::Tcp(stream)
    }
}

impl Read for NntpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            NntpStream::Tls(s) => s.read(buf),
            NntpStream::Tcp(s) => s.read(buf),
        }
    }
}

impl Write for NntpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            NntpStream::Tls(s) => s.write(buf),
            NntpStream::Tcp(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            NntpStream::Tls(s) => s.flush(),
            NntpStream::Tcp(s) => s.flush(),
        }
    }
}
