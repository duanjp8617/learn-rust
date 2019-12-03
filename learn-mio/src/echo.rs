use std::vec::Vec;
use std::env::Args;

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

impl<T> DummyCliParser<T> {

    pub fn new(default : T) -> DummyCliParser<T> {
        DummyCliParser {
            cli_info : default,
            pats : Vec::new(),
        }
    }

    fn search_for_matched_pattern(&mut self, new_pat: &String) -> Option<&mut CmdPat<T>> {        
        let mut iter_mut = self.pats.iter_mut();
        while let Some(pat) = iter_mut.next() {
            if pat.pat == *new_pat {
                return Some(pat);
            };
        };
        None
    }

    pub fn register_cmd_pat(&mut self, pat : String, need_arg : bool, cmd_type : CmdType, op : impl Fn(&mut T, String) + 'static) {
        if self.search_for_matched_pattern(&pat).is_none() {
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

    pub fn parse_args(&mut self, mut args : Args) -> bool {
        let mut parse_succeed = true;
        while let Some(arg) = args.next() {
            match self.search_for_matched_pattern(&arg) {
                Some(pat) => {
                    if pat.visited {
                        println!("duplicated argument {}", pat.pat);
                        return false;
                    }
                    pat.visited = true;
                    if pat.need_arg {
                        if let Some(next_arg) = args.next() {
                            (pat.op)(&mut self.cli_info, next_arg);
                        }
                        else {
                            println!("not enough arguments");
                            return false;
                        }
                    }
                    else {
                        (pat.op)(&mut self.cli_info, String::new());
                    }
                },
                None => {
                    println!("invalid arg name: {}", arg);
                    return false;
                }
            };

            /* let pat_iter = self.pats.iter_mut();
            for pat in pat_iter {
                if arg != pat.pat {
                    continue;
                }
                if pat.visited {
                    println!("duplicated argument {}", pat.pat);
                    parse_succeed = false;
                    break;
                }
                pat.visited = true;
                if pat.need_arg {
                    if let Some(next_arg) = args.next() {
                        (pat.op)(&mut self.cli_info, next_arg);
                    }
                    else {
                        println!("not enough arguments");
                        parse_succeed = false;
                        break;
                    }
                }
                else {
                    (pat.op)(&mut self.cli_info, String::new());
                }
            } */
        };
    }




}

pub fn run() {


    let mut fuck = DummyCliParser::<i32>::new(5);
    fuck.parse_args(std::env::args());
    
}