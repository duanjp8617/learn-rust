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
    op : Box<dyn Fn(&mut T, String)>,
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

    pub fn register_cmd_pat(&mut self, pat : String, need_arg : bool, cmd_type : CmdType, op : impl Fn(&mut T, String) + 'static) {
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
        while let Some(arg) = args.next() {
            match search_for_matched_pattern(&mut self.pats, &arg) {
                Some(pat) => {
                    if pat.visited {
                        return Err(format!("duplicated argument {}", &pat.pat));
                    }
                    pat.visited = true;
                    if pat.need_arg {
                        if let Some(next_arg) = args.next() {
                            (pat.op)(&mut self.cli_info, next_arg);
                        }
                        else {                            
                            return Err(format!("not enough arguments"));
                        }
                    }
                    else {
                        (pat.op)(&mut self.cli_info, String::new());
                    }
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
    });
    fuck.register_cmd_pat(String::from("-you"), false, CmdType::compulsory, |i: &mut i32, s : String|{
        if s.len() == 0 {
            println!("empty string");
        }
        else {
            println!("{}", &s);
        }
        *i = 7;
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