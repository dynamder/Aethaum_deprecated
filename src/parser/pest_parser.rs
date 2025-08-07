use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "./src/parser/aethaum.pest"]
pub struct AethaumParser;

mod test {
    use pest::iterators::{Pair, Pairs};
    use super::*;
    fn debug_parse_result(mut res: Pairs<Rule>) {
        recursive_printer(res.next().unwrap(),0)
    }
    fn depth_formatter(depth: usize) {
        for i in 0..depth {
            if i != depth - 1 {
                print!("\t");
            }else {
                print!("->");
            }
        }
    }
    fn recursive_printer(pair: Pair<Rule>, depth: usize) {
        let pair_clone = pair.clone();
        if pair_clone.into_inner().len() == 0 {
            depth_formatter(depth);
            println!("Rule: {:?}, Str: {}", pair.as_rule(), pair.as_str());
        }else {
            depth_formatter(depth);
            println!("Rule: {:?}, Str: {}", pair.as_rule(), pair.as_str());
            for inner_pair in pair.into_inner() {
                recursive_printer(inner_pair, depth + 1);
            }
            
        }
    }
    #[test]
    fn test_parser_c_expr1() { //c stands for should_correct,e stands for should_error
        let input = r#"
            1 + 3 / 2
        "#;
        let res = AethaumParser::parse(Rule::file, input).unwrap();
        debug_parse_result(res);
    }
    #[test]
    fn test_parser_c_expr2() {
        let input = r#"
            1 - 3 - 2 * ( 1 + 2 + 3    )
        "#;
        let res = AethaumParser::parse(Rule::file, input).unwrap();
        debug_parse_result(res);
    }
    #[test]
    fn test_parser_c_expr3() { 
        let input = r#"
            1 < 3 + 2
        "#;
        let res = AethaumParser::parse(Rule::file, input).unwrap();
        debug_parse_result(res);
    }
    #[test]
    #[should_panic]
    fn test_parser_e_expr1() {
        let input = r#"
            1 + 3 * 
        "#;
        let res = AethaumParser::parse(Rule::file, input).unwrap();
        debug_parse_result(res);
    }
}