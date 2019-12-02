use std::vec::Vec;
use std::marker::PhantomData;

struct CmdPat<T, F: Fn(&mut T, String)> {
    pat : String,
    need_arg : bool,
    visited : bool,
    op : F,
    marker : PhantomData<T>,
}

struct DummyCliParser<T, F: Fn(&mut T, String)> {
    cli_info : T,
    pats : Vec<CmdPat<T, F>>,
}

impl<T, F: Fn(&mut T, String)> DummyCliParser<T, F> {
    pub fn new(default : T) -> DummyCliParser<T, F> {
        DummyCliParser {
            cli_info : default,
            pats : Vec::new(),
        }
    }

    pub fn register_cmd_pat(&mut self, pat : String, need_arg : bool, op : F) {
        
    }


}

pub fn run() {
    let mut arg_iter = std::env::args();
    while let Some(arg) = arg_iter.next() {
        println!("{}", arg);
    }; 
    
}