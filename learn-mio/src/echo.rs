use std::vec::Vec;
use std::env::Args;
use std::result::Result;

enum CmdType {
    compulsory,
    have_default,
}

struct CmdPat<T> {
    pat : String,
    need_arg : bool,
    cmd_type : CmdType,
    op : Box<dyn Fn(&mut T, String) -> Option<String>>,
    visited : bool,
}

struct DummyCliParser<T> {
    cli_info : T,
    pats : Vec<CmdPat<T>>,
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

impl<T> DummyCliParser<T> {

    pub fn new(default : T) -> DummyCliParser<T> {
        DummyCliParser {
            cli_info : default,
            pats : Vec::new(),
        }
    }

    pub fn register_cmd_pat(&mut self, pat : String, need_arg : bool, cmd_type : CmdType, op : impl Fn(&mut T, String) -> Option<String> + 'static) {
        if search_for_matched_pattern(&mut self.pats, &pat).is_none() {
            self.pats.push(CmdPat {
                pat : pat,
                need_arg : need_arg,
                cmd_type : cmd_type,
                op : Box::new(op),
                visited : false,
            });
        }        
    }

    fn cmd_parsing_succeed(&self) -> bool{
        let iter = self.pats.iter();
        let mut succeed = true;
        for pat in iter {
            match &pat.cmd_type {
                compulsory => {
                    if !pat.visited {
                        succeed = false;
                    }
                    println!("missing argument {}", pat.pat);
                    break;
                },
                have_default => {},
            };
        };

        succeed
    }


    pub fn parse_args(&mut self, mut args : Args) -> Result<&T, String> {
        // the first argument is always the command name in linux
        args.next().unwrap();

        let func = |pat : &mut CmdPat<T>, ci : &mut T, args : &mut Args| -> Option<String> {
            match pat.visited {
                true => {
                    Some(format!("duplicated argument {}", &pat.pat))
                },
                false => {
                    pat.visited = true;
                    match pat.need_arg {
                        true => {
                            args.next().map_or_else(
                                || {
                                    Some(format!("not enough arguments"))
                                }, 
                                |next_arg| {
                                    (pat.op)(ci, next_arg)
                                }
                            )
                        },
                        false => {
                            (pat.op)(ci, String::new())
                        }
                    }
                }
            }
        };
        while let Some(arg) = args.next() {
            match search_for_matched_pattern(&mut self.pats, &arg) {
                Some(pat) => {
                    match func(pat, &mut self.cli_info, &mut args) {
                        Some(err_msg) => return Err(err_msg),
                        _ => {}, 
                    };
                },
                None => {                    
                    return Err(format!("invalid arg name: {}", arg));
                }
            };
        };
        return Ok(&self.cli_info);
    }

}

pub fn run() {

    let mut fuck = DummyCliParser::<i32>::new(5);
    fuck.register_cmd_pat(String::from("-fuck"), true, CmdType::compulsory, |i: &mut i32, s : String|{
        if s.len() == 0 {
            println!("empty string");
        }
        else {
            println!("{}", &s);
        }
        *i = 6;
        None
    });
    fuck.register_cmd_pat(String::from("-you"), false, CmdType::compulsory, |i: &mut i32, s : String|{
        if s.len() == 0 {
            println!("empty string");
        }
        else {
            println!("{}", &s);
        }
        *i = 7;
        None
    });

    match fuck.parse_args(std::env::args()) {
        Ok(val) => {
            println!("Parse succeed, with {}", val);
        },
        Err(s) => {
            println!("Parse fail with {}", &s);
        }
    };
}