use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest/json/json.pest"]
pub struct JsonParser;

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Array(Vec<JsonValue<'a>>),
    Object(Vec<(&'a str, JsonValue<'a>)>),
    Null,
}

pub fn root(content: &str) -> Result<JsonValue, Error<Rule>> {
    let json = JsonParser::parse(Rule::json, content)?.next().unwrap();
    Ok(parse_json_value(json))
}

pub fn parse_json_value(pair: Pair<Rule>) -> JsonValue {
    match pair.as_rule() {
        Rule::number => JsonValue::Number(pair.as_str().parse().unwrap()),
        Rule::string => JsonValue::String(pair.into_inner().next().unwrap().as_str()),
        Rule::boolean => JsonValue::Boolean(pair.as_str().parse().unwrap()),
        Rule::null => JsonValue::Null,
        Rule::array => JsonValue::Array(pair.into_inner().map(parse_json_value).collect()),
        Rule::object => JsonValue::Object(
            pair.into_inner()
                .map(|pair| {
                    let mut inner_rules = pair.into_inner();
                    let key = inner_rules
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str();
                    let value = parse_json_value(inner_rules.next().unwrap());
                    (key, value)
                })
                .collect()
        ),
        _ => unreachable!()
    }
}


pub fn serialize_json_value(val: &JsonValue) -> String {
    use JsonValue::*; // 方便后续枚举

    match val {
        Number(n) => format!("{}", n),
        String(s) => format!("\"{}\"", s),
        Boolean(b) => format!("{}", b),
        Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize_json_value).collect();
            format!("[{}]", contents.join(","))
        }
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(key, value)| format!("\"{}\":{}", key, serialize_json_value(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Null => "null".to_string(),
    }
}

#[cfg(test)]
mod test {
    use crate::pest::json::json::{JsonValue, root, serialize_json_value};

    #[test]
    fn test_parse_json_by_pest() {
        let data = "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { \"hello\" : \"world\"}
  } ";
        println!("will try to parse valid JSON data:\n\n**********\n{}\n**********\n", data);

        // will try to parse valid JSON data:
        //
        //     **********
        // { "a" : 42,
        //     "b": [ "x", "y", 12 ] ,
        //     "c": { "hello" : "world"}
        // }
        // **********


        let json_result: JsonValue = root(data).expect("unsuccessful JSON");

        println!("{}", serialize_json_value(&json_result))
        // {"a":42,"b":["x","y",12],"c":{"hello":"world"}}
    }
}





