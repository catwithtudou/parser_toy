use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;

fn parse_abc_or_def(input: &str) -> IResult<&str, &str> {
    alt((
        tag("abc"),
        tag("def"),
    ))(input)
}

#[test]
fn test_parse_abc_or_def() {
    let (leftover_input, output) = parse_abc_or_def("abcWorld").unwrap();
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "abc");
    let (_, output) = parse_abc_or_def("defWorld").unwrap();
    assert_eq!(output, "def");
    assert!(parse_abc_or_def("ghiWorld").is_err());
}