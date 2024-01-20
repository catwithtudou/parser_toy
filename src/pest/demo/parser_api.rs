use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest/demo/parser_api.pest"]
pub struct DemoParser;


#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::pest::demo::parser_api::{DemoParser, Rule};

    #[test]
    pub fn test_tokens() {
        let parse_result = DemoParser::parse(Rule::sum, "1773 + 1362").unwrap();
        let tokens = parse_result.tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    pub fn test_pairs() {
        let pair = DemoParser::parse(Rule::enclosed, "(..6472..) and more text")
            .unwrap().next().unwrap();

        assert_eq!(pair.as_rule(), Rule::enclosed);
        assert_eq!(pair.as_str(), "(..6472..)");

        let inner_rules = pair.into_inner();
        println!("{}", inner_rules); // --> [number(3, 7)]

        let pairs = DemoParser::parse(Rule::sum, "1773 + 1362")
            .unwrap().next().unwrap()
            .into_inner();

        let numbers = pairs
            .clone()
            .map(|pair| str::parse(pair.as_str()).unwrap())
            .collect::<Vec<i32>>();
        assert_eq!(vec![1773, 1362], numbers);

        for (found, expected) in pairs.zip(vec!["1773", "1362"]) {
            assert_eq!(Rule::number, found.as_rule());
            assert_eq!(expected, found.as_str());
        }
    }
}