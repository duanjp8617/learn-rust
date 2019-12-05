mod miotcp;
mod echo;
mod dummy_cli_parser;

use dummy_cli_parser::{DummyCliParser, PatternType};

fn main() {
    let mut fuck = DummyCliParser::<i32>::new(5);

    fuck.register_pattern(
        "-fuck", PatternType::WithArg, "cao ni ma", 
        |s : String, i: &mut i32|{
            if s.len() == 0 {
                println!("empty string");
            }
            else {
                println!("{}", &s);
            }
            *i = 6;
            Ok(())
        },
    ).unwrap();

    fuck.register_pattern("-you", PatternType::OptionalWithoutArg, "cao ni ma", 
        |s : String, i: &mut i32|{
            if s.len() == 0 {
                println!("empty string");
            }
            else {
                println!("{}", &s);
            }
            *i = 7;
            Ok(())
        },
    ).unwrap();

    let parse_obj;
    match fuck.parse_env_args() {
        Ok(val) => {
            parse_obj = val;            
        },
        Err(s) => {
            println!("{}", &s);
            return;
        }
    };
    println!("Parse succeed, with: {}", &parse_obj);
}
