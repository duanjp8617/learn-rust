use std::vec::Vec;
use std::env::Args;
use std::result::Result;

enum CmdType {
    Compulsory,
    HaveDefault,
}

struct CmdPat<T> {
    pat : String,
    need_arg : bool,
    cmd_type : CmdType,
    op : Box<dyn Fn(&mut T, String) -> Result<(), String>>,
    visited : bool,
}

struct DummyCliParser<T> {
    cli_info : T,
    pats : Vec<CmdPat<T>>,
    compulsory_cnt : i32,
}

fn search_for_matched_pattern<'a, T>(pats: &'a mut Vec<CmdPat<T>>, new_pat: &String) -> Option<&'a mut CmdPat<T>> {
    let mut iter_mut = pats.iter_mut();
    while let Some(pat) = iter_mut.next() {
        if pat.pat == *new_pat {
            return Some(pat);
        };
    };
    None
}

fn call_parse_func<T>(pat: &mut CmdPat<T>, cli_info: &mut T, args: &mut Args) -> Result<(), String> {
    match pat.visited {
        true => {
            Err(format!(
                concat!(r#"[DummyCliParser Error]: "#,
                        r#"argument pattern \"{}\" is duplicated."#), 
                &pat.pat
            ))
        },
        false => {
            pat.visited = true;
            match pat.need_arg {
                true => {
                    args.next().map_or_else(
                        || {
                            Err(format!(
                                concat!(r#"[DummyCliParser Error]: "#,
                                        r#"no argument for pattern \"{}\"."#), 
                                &pat.pat
                            ))
                        }, 
                        |next_arg| {
                            (pat.op)(cli_info, next_arg)
                        }
                    )
                },
                false => {
                    (pat.op)(cli_info, String::new())
                }
            }
        }
    }
}

impl<T> DummyCliParser<T> {

    pub fn new(default : T) -> DummyCliParser<T> {
        DummyCliParser {
            cli_info : default,
            pats : Vec::new(),
            compulsory_cnt : 0,
        }
    }

    pub fn register_cmd_pat(&mut self, pat : String, need_arg : bool, cmd_type : CmdType, op : impl Fn(&mut T, String) -> Result<(), String> + 'static) -> Result<(), String> {
        if search_for_matched_pattern(&mut self.pats, &pat).is_none() {
            match &cmd_type {
                CmdType::Compulsory => self.compulsory_cnt += 1,
                _ => {},
            };
            self.pats.push(CmdPat {
                pat : pat,
                need_arg : need_arg,
                cmd_type : cmd_type,
                op : Box::new(op),
                visited : false,
            });
            Ok(())
        }
        else {
            Err(format!("[DummyCliParser Error]: argument pattern \"{}\" is already registered.", &pat))
        }        
    }

    pub fn parse_args(mut self, mut args : Args) -> Result<T, String> {
        // the first argument is always the command name in linux
        args.next().unwrap();

        let mut cnt = 0;
        while let Some(arg) = args.next() {
            match search_for_matched_pattern(&mut self.pats, &arg) {
                Some(pat) => {
                    match call_parse_func(pat, &mut self.cli_info, &mut args) {
                        Err(err_msg) => return Err(err_msg),
                        _ => {
                            match &pat.cmd_type {
                                CmdType::Compulsory => cnt += 1,
                                _ => {},
                            }
                        }, 
                    };
                },
                None => {                    
                    return Err(format!("[DummyCliParser Error]: invalid argument pattern \"{}\"", arg));
                }
            };
        };
        if cnt == self.compulsory_cnt {
            return Ok(self.cli_info);
        }
        else {
            return Err(format!(
                "[DummyCliParser Error]: the number of compulsory argument patterns is {}, but only find {} in the argument list.", self.compulsory_cnt, cnt));
        }
    }

}

pub fn run() {

    let mut fuck = DummyCliParser::<i32>::new(5);

    fuck.register_cmd_pat(String::from("-fuck"), true, CmdType::Compulsory, |i: &mut i32, s : String|{
        if s.len() == 0 {
            println!("empty string");
        }
        else {
            println!("{}", &s);
        }
        *i = 6;
        Ok(())
    }).unwrap();

    fuck.register_cmd_pat(String::from("-you"), false, CmdType::HaveDefault, |i: &mut i32, s : String|{
        if s.len() == 0 {
            println!("empty string");
        }
        else {
            println!("{}", &s);
        }
        *i = 7;
        Ok(())
    }).unwrap();

    match fuck.parse_args(std::env::args()) {
        Ok(val) => {
            println!("Parse succeed, with: {}", &val);
        },
        Err(s) => {
            println!("{}", &s);
        }
    };
}