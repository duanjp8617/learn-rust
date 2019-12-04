use std::vec::Vec;
use std::result::Result;

// pattern, arg, pattern (arg pattern), arg -> arg list

pub enum PatternType {
    Compulsory,
    HaveDefault,
}

pub struct Pattern<T> {
    pub pat_str : String,
    pub need_arg : bool, 
    pub pat_type : PatternType, 
    pub parse_func : Box<dyn Fn(String, &mut T) -> Result<(), String>>,
    pub description : String,
}

struct Pat<T> {
    internal : Pattern<T>,
    visited : bool,
}

impl<T> Pat<T> {
    fn call_parse_func<I>(&mut self, parse_obj: &mut T, args: &mut I) -> Result<(), String> where I: Iterator<Item = String> {
        match self.visited {
            true => {
                Err(format!("[DummyCliParser Error]: argument pattern \"{}\" is duplicated.",  &self.internal.pat_str))
            },
            false => {
                self.visited = true;
                match self.internal.need_arg {
                    true => {
                        args.next().map_or_else(
                            || {
                                Err(format!("[DummyCliParser Error]: no argument for pattern \"{}\".",&self.internal.pat_str))
                            }, 
                            |next_arg| {
                                (self.internal.parse_func)(next_arg, parse_obj)
                            }
                        )
                    },
                    false => {
                        (self.internal.parse_func)(String::new(), parse_obj)
                    }
                }
            }
        }
    }
}

pub struct DummyCliParser<T> {
    parse_obj : T,
    pats : Vec<Pat<T>>,
    compulsory_cnt : i32,
}

fn search_for_matched_pattern<'a, T>(pats: &'a mut Vec<Pat<T>>, pat_str : &String) -> Option<&'a mut Pat<T>> {
    let mut iter_mut = pats.iter_mut();
    while let Some(pat) = iter_mut.next() {
        if pat.internal.pat_str == *pat_str {
            return Some(pat)
        }
    }
    None
}

impl<T> DummyCliParser<T> {

    pub fn new(parse_obj : T) -> DummyCliParser<T> {
        DummyCliParser {
            parse_obj : parse_obj,
            pats : Vec::new(),
            compulsory_cnt : 0,
        }
    }
    
    pub fn register_cmd_pat(&mut self, pat : Pattern<T>) -> Result<(), String> {
        if search_for_matched_pattern(&mut self.pats, &pat.pat_str).is_none() {
            match &pat.pat_type {
                PatternType::Compulsory => self.compulsory_cnt += 1,
                _ => {},
            };
            self.pats.push(Pat {
                internal : pat,
                visited : false,
            });
            Ok(())
        }
        else {
            Err(format!("[DummyCliParser Error]: argument pattern \"{}\" is already registered.", &pat.pat_str))
        }        
    }

    fn do_parse_args<I>(mut self, mut args : I) -> Result<T, String> where I: Iterator<Item = String>{        
        let mut cnt = 0;
        while let Some(arg_str) = args.next() {
            match search_for_matched_pattern(&mut self.pats, &arg_str) {
                Some(pat) => {
                    match pat.call_parse_func(&mut self.parse_obj, &mut args) {
                        Err(err_msg) => return Err(err_msg),
                        _ => {
                            match &pat.internal.pat_type {
                                PatternType::Compulsory => cnt += 1,
                                _ => {},
                            }
                        }, 
                    };
                },
                None => {                    
                    return Err(format!("[DummyCliParser Error]: invalid argument pattern \"{}\"", arg_str));
                }
            };
        };
        if cnt == self.compulsory_cnt {
            return Ok(self.parse_obj);
        }
        else {
            return Err(format!(
                concat!(
                    r#"[DummyCliParser Error]: the number of compulsory argument patterns is {}, "#,
                    r#"but only find {} in the argument list."#), 
                self.compulsory_cnt, cnt
            ));
        }
    }

    fn build_help_string(&self) -> String {
        let mut res = String::with_capacity(512);
        for pat in &self.pats {
            let first_part = if pat.internal.need_arg {
                format!("{} arg", &pat.internal.pat_str)
            }
            else {
                format!("{}", &pat.internal.pat_str)
            };
            match &pat.internal.pat_type {
                PatternType::Compulsory => {
                    let new_str = res + &format!("{}: {}\n", &first_part, &pat.internal.description);
                    res = new_str;
                }
                _ => {
                    let new_str = res + &format!("[{}]: {}\n", &first_part, &pat.internal.description);
                    res = new_str;
                }
            }
        };
        res
    }

    // this function assumes that the first argument 
    // returned by std::env::args() is always the invoked command
    pub fn parse_env_args(self) -> Result<T, String> {
        let mut iter_mut = std::env::args();
        iter_mut.next().unwrap();
        
        match iter_mut.next() {
            Some(arg_str) => {
                if arg_str == String::from("-h") || arg_str == String::from("--help") {                
                    let mut help_str = self.build_help_string();
                    help_str.pop();
                    Err(help_str)
                }
                else {
                    iter_mut = std::env::args();
                    iter_mut.next().unwrap();
                    self.do_parse_args(iter_mut)
                }
            },
            _ => {
                self.do_parse_args(iter_mut)
            }
        }
    }
}