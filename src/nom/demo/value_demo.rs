use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;

fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((
        value(true, tag("true")),
        value(false, tag("false")),
    ))(input)
}

#[test]
fn test_parse_bool() {
    let (remaining, parsed) = parse_bool("true|false").unwrap();
    assert_eq!(parsed, true);
    assert_eq!(remaining, "|false");
    assert!(parse_bool(remaining).is_err());
}
