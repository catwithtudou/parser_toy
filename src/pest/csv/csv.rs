use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest/csv/csv.pest"]
pub struct CSVParser;


#[cfg(test)]
mod test {
    use std::fs;

    use pest::Parser;

    use crate::pest::csv::csv::{CSVParser, Rule};

    #[test]
    pub fn test_parse() {
        let successful_parse = CSVParser::parse(Rule::field, "-273.15");
        println!("{:?}", successful_parse);

        let unsuccessful_parse = CSVParser::parse(Rule::field, "China");
        println!("{:?}", unsuccessful_parse);
    }

    #[test]
    pub fn test_csv() {
        let unparsed_file = fs::read_to_string("src/pest/csv/numbers.csv").expect("cannot read file");

        let file = CSVParser::parse(Rule::file, &unparsed_file)
            .expect("unsuccessful parse")
            .next().unwrap();


        let mut field_sum: f64 = 0.0;
        let mut record_count: u64 = 0;

        for record in file.into_inner() {
            match record.as_rule() {
                Rule::record => {
                    record_count += 1;

                    for field in record.into_inner() {
                        field_sum += field.as_str().parse::<f64>().unwrap();
                    }
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        println!("Sum of fields:{}", field_sum);
        print!("Number of records:{}", record_count);
    }
}
