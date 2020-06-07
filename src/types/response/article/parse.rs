use crate::types::response::Headers;
use nom::branch::alt;
use nom::bytes::complete as bc;
use nom::bytes::complete::{is_a, take, take_while, take_while1, take_while_m_n};
use nom::character::complete::{anychar, char, crlf, space0, space1};
use nom::character::is_alphabetic;
use nom::combinator::{iterator, map, map_parser, opt, verify};
use nom::lib::std::str::{from_utf8, Utf8Error};
use nom::multi::{fold_many1, many0};
use nom::sequence::{terminated, tuple};
use nom::IResult;
use std::collections::HashMap;

/// Returns true if the character is any ASCII non-control character other than a colon
///
/// [A-NOTCOLON](https://tools.ietf.org/html/rfc3977#section-9.8)
fn is_a_notcolon(chr: u8) -> bool {
    (chr >= 0x21 && chr <= 0x39) || (chr >= 0x3b && chr <= 0x7e)
}

/// Returns true if the slice is UTF-8 and contains no ascii characters
///
/// [UTF8-non-ascii](https://tools.ietf.org/html/rfc3977#section-9.8)
fn is_utf8_non_ascii(b: &[u8]) -> bool {
    if let Ok(s) = from_utf8(b) {
        // if any bytes are ascii this fails the test
        !s.bytes().any(|u| u.is_ascii())
    } else {
        false
    }
}

/// Returns true if the char is any ASCII character from `!` through `~`
///
/// [`A-CHAR`](https://tools.ietf.org/html/rfc3977#section-9.8)
fn is_a_char(chr: u8) -> bool {
    (chr >= 0x21 && chr <= 0x7e)
}

/// Returns true if the byte slice is a *single* non ASCII non-control char
///
/// [`A-CHAR`](https://tools.ietf.org/html/rfc3977#section-9.8)
fn is_a_char_bytes(b: &[u8]) -> bool {
    if b.len() > 1 {
        false
    } else {
        is_a_char(b[0])
    }
}

/// Take an A-CHAR from the slice
fn take_a_char(b: &[u8]) -> IResult<&[u8], &[u8]> {
    verify(take_ascii_byte, is_a_char_bytes)(b)
}

/// Take a single non-ascii UTF-8 character from the slice
///
/// nom 5 lacks combinators to distinguish between ASCII and UTF-8 so we have to implement this
/// manually
///
///
/// [`UTF8-non-ascii`](https://tools.ietf.org/html/rfc3977#section-9.8)
fn take_utf8_non_ascii(b: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        verify(take(1u8), is_utf8_non_ascii),
        verify(take(2u8), is_utf8_non_ascii),
        verify(take(3u8), is_utf8_non_ascii),
        verify(take(4u8), is_utf8_non_ascii),
    ))(b)
}

/// Take a single `A-CHAR` or `UTF8-non-ascii` from the slice
/// ```abnf
/// P-CHAR     = A-CHAR / UTF8-non-ascii
/// A-CHAR     = %x21-7E
/// ```
fn take_p_char(b: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((take_a_char, take_utf8_non_ascii))(b)
}

/// Take the header-name from a slice
///
/// The header-name is defined as 1 or more `A-NOTCOLON` characters
///
/// [header-name](https://tools.ietf.org/html/rfc3977#section-9.8)
fn take_header_name(b: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_a_notcolon)(b)
}

/// A token is one or more `P-CHAR` characters
///
/// [token](https://tools.ietf.org/html/rfc3977#section-9.8)
fn take_token(b: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, token_len) = fold_many1(take_p_char, 0, |mut acc, slice| {
        acc += slice.len();
        acc
    })(b)?;

    let token = &b[..token_len];
    Ok((rest, token))
}

/// Take a single byte
///
/// This combinator simply returns a single byte if it is ASCII
fn take_ascii_byte(b: &[u8]) -> IResult<&[u8], &[u8]> {
    verify(take(1u8), |uint: &[u8]| uint.is_ascii())(b)
}

/// The content of an Article Header
///
/// Headers may be split across multiple lines (aka folded)
///
/// [RFC 3977 Appendix 1](https://tools.ietf.org/html/rfc3977#appendix-A.1)
///
/// ```abnf
/// header-content = [WS] token *( [CRLF] WS token )
/// ```
fn take_header_content(b: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, (_ws, token, more_tokens)) = tuple((
        opt(space0),
        take_token,
        many0(tuple((opt(crlf), space1, take_token))),
    ))(b)?;

    let bytes_read = b.len() - rest.len();
    Ok((rest, &b[..bytes_read]))
}

/// https://tools.ietf.org/html/rfc3977#appendix-A.1
///
/// ```abnf
/// header = header-name ":" SP [header-content] CRLF
/// header-content = [WS] token *( [CRLF] WS token )
/// ```
fn take_header(b: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    // he
    let (rest, (header_name, _, _, header_content)) = terminated(
        tuple((take_header_name, char(':'), char(' '), take_header_content)),
        crlf,
    )(b)?;
    Ok((rest, (header_name, header_content)))
}

pub(crate) fn take_headers(b: &[u8]) -> IResult<&[u8], HashMap<String, Vec<String>>> {
    // n.b. assuming there are no parsing bugs (big if there), it should be sound to use
    // from_utf8_unchecked here since we already did utf8 checks while parsing.

    let fold_headers = fold_many1(take_header, HashMap::new(), |mut acc, (name, content)| {
        let name = String::from_utf8_lossy(name).to_string();
        let content = String::from_utf8_lossy(content).to_string();
        let entries: &mut Vec<_> = acc.entry(name).or_default();
        entries.push(content);

        acc
    });

    let (rest, header_map) = terminated(fold_headers, crlf)(b)?;
    Ok((rest, header_map))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEXT_ARTICLE: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/text_article"));

    const FOLDED_HEADER: &[u8; 127] =
        b"X-Received: by 2002:ac8:2aed:: with SMTP id c42mr5587158qta.202.1591290821135;\r\n        \
            Thu, 05 Jun 2020 10:13:41 -0700 (PDT)\r\n";

    mod is_utf8_non_ascii {
        use super::*;

        #[test]
        fn happy_path() {
            ["🤘".as_bytes(), "¥".as_bytes(), "🚃".as_bytes()]
                .iter()
                .for_each(|b| {
                    println!("Testing `{}` -- {:?}", from_utf8(b).unwrap(), b);
                    assert_eq!(is_utf8_non_ascii(b), true)
                });
        }

        #[test]
        fn fail_ascii() {
            assert_eq!(is_utf8_non_ascii("1".as_bytes()), false)
        }
    }

    mod take_utf8_non_ascii {
        use super::*;

        #[test]
        fn happy_path() {
            ["🤘", "¢", "ぎ", "🚃5"].iter().for_each(|s| {
                println!("Testing `{}` -- {:?}", s, s.as_bytes());
                let (rest, byte) = take_utf8_non_ascii(s.as_bytes()).unwrap();

                assert_eq!(from_utf8(byte).unwrap().chars().next(), s.chars().next());
                assert_eq!(rest.is_empty(), s.chars().count() == 1);
            })
        }
    }

    #[test]
    fn test_token() {
        let (rest, token) = take_token("📯1🤘 some words 🐒 ".as_bytes()).unwrap();
        dbg!(from_utf8(rest));
        dbg!(from_utf8(token));

        assert_eq!(token, "📯1🤘".as_bytes());
        assert_eq!(rest, " some words 🐒 ".as_bytes())
    }

    mod take_ascii_byte {
        use super::*;
        #[test]
        fn happy_path() {
            let (rest, char) = take_ascii_byte("5".as_bytes()).unwrap();
        }
        #[test]
        fn fail_on_unicode() {
            assert!(take_ascii_byte("🤘 ".as_bytes()).is_err());
        }
    }

    #[test]
    fn test_take_header_name() {
        let (rest, header_name) = take_header_name(FOLDED_HEADER).unwrap();
        assert_eq!(header_name, b"X-Received");
        assert_ne!(rest.len(), 0);
    }

    #[test]
    fn test_header_content() {
        let content =
            b"by 2002:ac8:2aed:: with SMTP id c42mr5587158qta.202.1591290821135;\r\n        \
            Thu, 05 Jun 2020 10:13:41 -0700 (PDT)\r\n";

        let (rest, parsed_header) = take_header_content(&content[..]).unwrap();

        // header-content does include the final CRLF, that's part of the header
        assert_eq!(&content[..content.len() - 2], parsed_header)
    }

    mod test_take_header {
        use super::*;

        #[test]
        fn test_folded() {
            let content =
                b"by 2002:ac8:2aed:: with SMTP id c42mr5587158qta.202.1591290821135;\r\n        \
            Thu, 05 Jun 2020 10:13:41 -0700 (PDT)\r\n";

            let (rest, (header_name, parsed_content)) = take_header(FOLDED_HEADER).unwrap();
            dbg!(from_utf8(&header_name).unwrap());
            dbg!(from_utf8(&rest).unwrap());
            assert_eq!(rest.len(), 0);
            assert_eq!(header_name, &b"X-Received"[..]);
            assert_eq!(parsed_content, &content[..content.len() - 2])
        }

        #[test]
        fn test_simple() {
            let header = "Xref: number.nntp.giganews.com mozilla.dev.platform:47661\r\n";

            let (rest, (name, content)) = take_header(header.as_bytes()).unwrap();

            assert_eq!(rest.len(), 0);
            assert_eq!(name, header.split(':').next().unwrap().as_bytes());
            assert_eq!(
                from_utf8(content).unwrap(),
                header
                    .splitn(2, ':')
                    .skip(1)
                    .next()
                    .map(|s| s.trim())
                    .unwrap()
            )
        }
    }

    #[test]
    fn test_take_headers() {
        // strip the initial response line
        let article = TEXT_ARTICLE.splitn(2, '\n').skip(1).next().unwrap();
        let (rest, headers) = take_headers(article.as_bytes()).unwrap();

        println!("{:#?}", headers);

        assert!(rest.starts_with(b"In bug 1630935 [1], I intend to deprecate support for drawing"));
        assert!(headers.contains_key("X-Received"));
        assert_eq!(headers.get("X-Received").unwrap().len(), 2);
    }
}
