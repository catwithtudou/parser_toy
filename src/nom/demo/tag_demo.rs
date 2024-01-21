use nom::bytes::complete::tag;
use nom::IResult;

pub fn parse_input(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_input() {
        let (leftover_input, output) = parse_input("abcWorld!").unwrap();
        assert_eq!(leftover_input, "World!");
        assert_eq!(output, "abc");
        assert!(parse_input("defWorld").is_err());
    }
}



