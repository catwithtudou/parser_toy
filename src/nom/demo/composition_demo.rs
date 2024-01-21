use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::IResult;
use nom::sequence::tuple;

fn parse_base(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("a"), // 与 tag 相比不区分大小写的标签
        tag_no_case("t"),
        tag_no_case("c"),
        tag_no_case("g"),
    ))(input)
}

fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((
        parse_base, parse_base
    ))(input)
}

#[test]
fn test_parse_pair() {
    let (remaining, parsed) = parse_pair("aTcG").unwrap();
    assert_eq!(parsed, ("a", "T"));
    assert_eq!(remaining, "cG");
    assert!(parse_pair("Dct").is_err());
}