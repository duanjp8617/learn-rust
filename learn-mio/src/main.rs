mod miotcp;
mod echo;
mod dummy_cli_parser;

use dummy_cli_parser::{DummyCliParser, Pattern, PatternType};

fn main() {
    let mut fuck = DummyCliParser::<i32>::new(5);

    fuck.register_cmd_pat(Pattern{
        pat_str : String::from("-fuck"), 
        need_arg : true, 
        pat_type : PatternType::Compulsory, 
        parse_func : Box::new(|s : String, i: &mut i32|{
            if s.len() == 0 {
                println!("empty string");
            }
            else {
                println!("{}", &s);
            }
            *i = 6;
            Ok(())
        }),
        description : String::from("cao ni ma"),
    }).unwrap();

    fuck.register_cmd_pat(Pattern{
        pat_str : String::from("-you"), 
        need_arg : false, 
        pat_type : PatternType::HaveDefault, 
        parse_func : Box::new(|s : String, i: &mut i32|{
            if s.len() == 0 {
                println!("empty string");
            }
            else {
                println!("{}", &s);
            }
            *i = 6;
            Ok(())
        }),
        description : String::from("cao ni ma ya"),
    }).unwrap();

    match fuck.parse_env_args() {
        Ok(val) => {
            println!("Parse succeed, with: {}", &val);
        },
        Err(s) => {
            println!("{}", &s);
        }
    };
}
