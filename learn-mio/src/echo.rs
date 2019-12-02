struct CmdPat<T, F: FnMut(&mut T, String)> {
    pat : String,
    need_arg : bool,
    visited : bool,
    op : F,
    marker : std::marker::PhantomData<T>,
}

struct DummyCliParser<T> {
    cli_info : T,

}

pub fn run() {
    let mut arg_iter = std::env::args();
    while let Some(arg) = arg_iter.next() {
        println!("{}", arg);
    }; 
    
}