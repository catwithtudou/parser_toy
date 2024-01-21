use nom::{AsChar, IResult};
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while_m_n;
use nom::combinator::map_res;
use nom::sequence::tuple;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}


// 是否为16进制数字
pub fn is_hex_digit(c: char) -> bool {
    c.is_hex_digit()
}

// 将字符串转换为十进制结果
pub fn to_num(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

// 按 is_hex_digit 规则对输入按两位进行匹配，并将结果按 to_hex_num 转换十进制结果
pub fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        to_num,
    )(input)
}

// 十六进制颜色的解析器
pub fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_color() {
        assert_eq!(hex_color("#2F14DF"), Ok(("", Color {
            red: 47,
            green: 20,
            blue: 223,
        })))
    }
}