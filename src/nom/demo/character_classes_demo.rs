use nom::character::complete::alpha0;
use nom::IResult;

fn parse_alpha(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

#[test]
fn test_parse_alpha() {
    let (remaining, letters) = parse_alpha("abc123").unwrap();
    assert_eq!(remaining, "123");
    assert_eq!(letters, "abc");
}