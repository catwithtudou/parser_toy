use nom::bytes::complete::tag;
use nom::IResult;
use nom::multi::many0;

fn repeat_parser(s: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("abc"))(s)
}

#[test]
fn test_repeat_parser() {
    assert_eq!(repeat_parser("abcabc"), Ok(("", vec!["abc", "abc"])));
    assert_eq!(repeat_parser("abc123"), Ok(("123", vec!["abc"])));
    assert_eq!(repeat_parser("123123"), Ok(("123123", vec![])));
    assert_eq!(repeat_parser(""), Ok(("", vec![])));
}
