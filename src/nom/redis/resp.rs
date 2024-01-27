use std::fmt::{Display, Result};

use bytes::BytesMut;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::{char, digit1};
use nom::combinator::{cut, map};
use nom::IResult;
use nom::multi::many_m_n;
use nom::sequence::{delimited, preceded, terminated};

#[derive(Debug, Clone, PartialEq)]
pub enum Resp {
    StringLine(String),
    Err(String),
    Int(i64),
    Batch(Option<String>),
    MultiBatch(Option<Vec<Resp>>),
    BadReply(String),
}

impl Resp {
    pub fn from_resp(src: &BytesMut) -> Self {
        debug!("{:?}", src);
        match parse(&String::from_utf8(src.as_ref().to_vec()).unwrap()) {
            Ok((remain, resp)) => {
                if remain.is_empty() {
                    resp
                } else {
                    Resp::BadReply(format!("remaining bytes: {}", remain))
                }
            }
            Err(e) => Resp::BadReply(e.to_string()),
        }
    }
}

impl Display for Resp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            Resp::StringLine(line) => write!(f, "+ {}", line),
            Resp::Err(err) => write!(f, "- {}", err),
            Resp::Int(int) => write!(f, ": {}", int),
            Resp::Batch(reply) => {
                if let Some(reply) = reply {
                    write!(f, "$ {}", reply)
                } else {
                    write!(f, "$-1")
                }
            }
            Resp::MultiBatch(replies) => {
                if let Some(replies) = replies {
                    write!(
                        f,
                        "* {}\r\n{}",
                        replies.len(),
                        replies
                            .iter()
                            .map(|r| format!("{}", r))
                            .collect::<Vec<String>>()
                            .join("\r\n")
                    )
                } else {
                    write!(f, "*-1")
                }
            }
            Resp::BadReply(err) => write!(f, "parse reply failed: {}", err),
        }
    }
}


pub fn parse(i: &str) -> IResult<&str, Resp> {
    alt((
        parse_single_line,
        parse_err,
        parse_int,
        parse_batch,
        parse_multi_batch
    ))(i)
}


pub fn parse_single_line(i: &str) -> IResult<&str, Resp> {
    preceded(char('+'), cut(terminated(
        map(
            take_while1(|c: char| c != '\r' && c != '\n'),
            |resp: &str| Resp::StringLine(resp.to_string()),
        ),
        tag("\r\n"),
    )))(i)
}

pub fn parse_err(i: &str) -> IResult<&str, Resp> {
    preceded(char('-'), cut(terminated(
        map(
            take_while1(|c: char| c != '\r' && c != '\n'),
            |resp: &str| Resp::Err(resp.to_string()),
        ),
        tag("\r\n"),
    )))(i)
}

pub fn parse_int(i: &str) -> IResult<&str, Resp> {
    preceded(char(':'), cut(terminated(
        map(
            take_while1(|c: char| c.is_digit(10) || c == '-'),
            |resp: &str| Resp::Int(resp.parse::<i64>().unwrap()),
        ),
        tag("\r\n"),
    )))(i)
}


pub fn parse_batch(i: &str) -> IResult<&str, Resp> {
    preceded(char('$'), cut(alt((
        preceded(char('-'), cut(terminated(
            map(
                take_while1(|c: char| c.is_digit(10)),
                |_| Resp::Batch(None),
            ),
            tag("\r\n"),
        ))),
        preceded(preceded(digit1, tag("\r\n")), cut(terminated(
            map(
                take_while(|c: char| c != '\r' && c != '\n'),
                |resp: &str| Resp::Batch(Some(resp.to_string())),
            ),
            tag("\r\n"),
        )))
    ))))(i)
}

pub fn parse_multi_batch(i: &str) -> IResult<&str, Resp> {
    let (i, count) = delimited(
        tag("*"),
        map(
            take_while1(|c: char| c.is_digit(10) || c == '-'),
            |resp: &str| resp.parse::<i64>().unwrap(),
        ),
        tag("\r\n"))(i)?;
    if count == -1 {
        let (i, _) = tag("\r\n")(i)?;
        return Ok((i, Resp::MultiBatch(None)));
    }

    let count = count as usize;
    let (i, responses) = many_m_n(
        count,
        count,
        alt((parse_single_line, parse_err, parse_int, parse_batch)),
    )(i)?;
    if responses.len() != count {
        return Ok((
            i,
            Resp::BadReply(format!("expect {} items, got {}", count, responses.len())),
        ));
    }
    return Ok((i, Resp::MultiBatch(Some(responses))));
}


#[cfg(test)]
mod test {
    use crate::nom::redis::resp::*;

    #[test]
    fn test_parse_single_line() {
        let (_, resp) = parse_single_line("+OK\r\n").unwrap();

        if let Resp::StringLine(ok) = resp {
            assert_eq!(ok, "OK".to_string())
        }
    }

    #[test]
    fn test_parse_err() {
        let (_, resp) = parse_err("-Error\r\n").unwrap();

        if let Resp::Err(err) = resp {
            assert_eq!(err, "Error".to_string())
        }
    }

    #[test]
    fn test_parse_int() {
        let (_, resp) = parse_int(":-1\r\n").unwrap();

        if let Resp::Int(int) = resp {
            assert_eq!(int, -1)
        }
    }

    #[test]
    fn test_parse_batch() {
        let (_, none_resp) = parse_batch("$-1\r\n").unwrap();
        if let Resp::Batch(none) = none_resp {
            assert_eq!(none, None);
        }
        let (_, value_resp) = parse_batch("$6\r\nfoobar\r\n").unwrap();
        if let Resp::Batch(value) = value_resp {
            assert_eq!(value, Some("foobar".to_string()));
        }
        let (_, value_resp) = parse_batch("$0\r\n\r\n").unwrap();
        if let Resp::Batch(value) = value_resp {
            assert_eq!(value, Some("".to_string()));
        }
    }

    #[test]
    fn test_parse_multi_batch() {
        let (_, none_resp) = parse_multi_batch("*0\r\n").unwrap();
        if let Resp::MultiBatch(responses) = none_resp {
            assert_eq!(responses.unwrap().len(), 0);
        }

        let (_, value_resp) = parse_multi_batch("*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n").unwrap();
        if let Resp::MultiBatch(responses) = value_resp {
            assert_eq!(responses.clone().unwrap().len(), 2);
            assert_eq!(responses, Some(vec![Resp::Batch(Some("foo".to_string())), Resp::Batch(Some("bar".to_string()))]));
        }
    }
}