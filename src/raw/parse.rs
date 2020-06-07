use std::convert::TryInto;

use nom::bytes::complete::take_until;
use nom::character::complete::{crlf, one_of};
use nom::combinator::all_consuming;
use nom::sequence::{terminated, tuple};
use nom::IResult;

/// The first line of an NNTP response
///
/// This struct contains data borrowed from a read buffer
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct InitialResponseLine<'a> {
    /// The response code
    pub code: &'a [u8; 3],
    /// The data within the response NOT including leading whitespace and terminator characters
    pub data: &'a [u8],
    /// The entire response including the response code and termination characters
    pub buffer: &'a [u8],
}

/// Return true if the first character is a digit
fn one_of_digit(b: &[u8]) -> IResult<&[u8], char> {
    one_of("0123456789")(b)
}

/// Takes a line from the input buffer
///
/// A "line" is a sequence of bytes terminated by a CRLF (`\r\n`) sequence.
fn take_line(b: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, line) = terminated(take_until("\r\n"), crlf)(b)?;

    Ok((rest, line))
}

/// Takes a response code from the buffer
///
/// A valid response code is three ASCII digits where the first digit is between 1 and 5
fn take_response_code(b: &[u8]) -> IResult<&[u8], &[u8]> {
    let res: IResult<_, (char, char, char)> =
        tuple((one_of("12345"), one_of_digit, one_of_digit))(b);
    let (rest, _) = res?;

    Ok((rest, &b[0..3]))
}

/// Returns true if the buffer only contains a `.`
pub(crate) fn is_end_of_datablock(b: &[u8]) -> bool {
    b == b"."
}

/// Parse an first line of an NNTP response
///
/// Per [RFC 3977](https://tools.ietf.org/html/rfc3977#section-3.2), the first line of an
/// NNTP response consists of a three-digit response code, a single space, and then
/// some text terminated with a CRLF.
pub(crate) fn parse_first_line(b: &[u8]) -> IResult<&[u8], InitialResponseLine<'_>> {
    let res = all_consuming(tuple((
        take_response_code,
        nom::character::complete::char(' '),
        take_until("\r\n"),
        crlf,
    )))(b)?;

    let (rest, (code, _, data, _crlf)) = res;
    let code = code
        .try_into()
        .expect("Code should be three bytes, there is likely a bug in the parser.");

    Ok((
        rest,
        InitialResponseLine {
            code,
            data,
            buffer: b,
        },
    ))
}

/// Parse a data block line from the buffer
pub(crate) fn parse_data_block_line(b: &[u8]) -> IResult<&[u8], &[u8]> {
    all_consuming(take_line)(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::error::ErrorKind;
    use nom::Err;

    const MOTD: &[u8] =
        b"200 news.example.com InterNetNews server INN 2.5.5 ready (transit mode)\r\n";
    const MOTD_NO_CRLF: &[u8] =
        b"200 news.example.com InterNetNews server INN 2.5.5 ready (transit mode)";
    const CRLF: &[u8] = b"\r\n";

    mod test_parse_initial_response {
        use super::*;

        #[test]
        fn happy_path() {
            let (_remainder, raw_response) = parse_first_line(MOTD).unwrap();
            let expected_resp = InitialResponseLine {
                code: b"200",
                data: &b"news.example.com InterNetNews server INN 2.5.5 ready (transit mode)"[..],
                buffer: &MOTD,
            };
            assert_eq!(raw_response, expected_resp)
        }

        #[test]
        fn test_remaining_data() {
            let data = [MOTD, &b"SOME MORE DATA\r\n"[..]].concat();

            assert!(parse_first_line(&data).is_err());
        }
    }

    mod test_take_line {
        use super::*;

        #[test]
        fn happy_path() {
            assert_eq!(take_line(MOTD), Ok((&b""[..], MOTD_NO_CRLF)));
        }
    }

    mod test_parse_data_block {
        use super::*;

        #[test]
        fn happy_path() {
            let msg = b"101 Capability list:\r\n";
            let (_remainder, block) = parse_data_block_line(msg).unwrap();
            assert_eq!(block, b"101 Capability list:")
        }
    }

    mod test_parse_response_code {
        use super::*;

        #[test]
        fn happy_path() {
            [
                &b"200"[..],
                &b"200 "[..],
                &b"2000"[..],
                &b"200000"[..],
                &b"200123"[..],
                &b"200abc"[..],
            ]
            .iter()
            .for_each(|input| {
                let res = take_response_code(input);
                assert!(res.is_ok());
                let (_rest, code) = res.unwrap();
                assert_eq!(code, b"200")
            });
        }

        #[test]
        fn too_short() {
            println!("Testing {:?}", b"5");
            assert_eq!(
                take_response_code(b"5"),
                Err(Err::Error((&b""[..], ErrorKind::OneOf)))
            )
        }

        #[test]
        fn not_enough_digits() {
            assert_eq!(
                take_response_code(b"5ab500"),
                Err(Err::Error((&b"ab500"[..], ErrorKind::OneOf)))
            )
        }
    }
}
