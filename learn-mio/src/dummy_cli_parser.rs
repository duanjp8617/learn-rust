use std::vec::Vec;
use std::result::Result;

// pattern, arg, pattern (arg pattern), arg -> arg list

pub enum PatternType {
    WithArg,
    WithoutArg,
    OptionalWithArg,
    OptionalWithoutArg,
}

fn need_arg(pat_type: &PatternType) -> bool {
    match &pat_type {
        PatternType::WithArg => true,
        PatternType::WithoutArg => false,
        PatternType::OptionalWithArg => true,
        PatternType::OptionalWithoutArg => false,
    }
}

struct PatInternal<T> {
    pat_str : String,
    pat_type : PatternType, 
    parse_func : Box<dyn Fn(String, &mut T) -> Result<(), String>>,
    description : String,
}

struct Pat<T> {
    internal : PatInternal<T>,
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
                match need_arg(&self.internal.pat_type) {
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
    
    pub fn register_pattern(&mut self, pat: &'static str, pat_type: PatternType, description: &'static str,
                            parse_func : impl Fn(String, &mut T) -> Result<(), String> + 'static) -> Result<(), String> {
        let pat_str = String::from(pat);
        let description_str = String::from(description);
        if search_for_matched_pattern(&mut self.pats, &pat_str).is_none() {
            match &pat_type {
                PatternType::WithArg | PatternType::WithoutArg => self.compulsory_cnt += 1,
                _ => {},
            };
            self.pats.push(Pat {
                internal : PatInternal {
                    pat_str : pat_str,
                    pat_type : pat_type,
                    parse_func : Box::new(parse_func),
                    description : description_str,
                },
                visited : false,
            });
            Ok(())
        }
        else {
            Err(format!("[DummyCliParser Error]: argument pattern \"{}\" is already registered.", &pat_str))
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
                                PatternType::WithArg | PatternType::WithoutArg => cnt += 1,
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
            let first_part = if need_arg(&pat.internal.pat_type) {
                format!("{} arg", &pat.internal.pat_str)
            }
            else {
                format!("{}", &pat.internal.pat_str)
            };
            match &pat.internal.pat_type {
                PatternType::WithArg | PatternType::WithoutArg => {
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