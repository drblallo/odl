#![feature(proc_macro_hygiene)]
extern crate plex;

use odl::lexer::IndentLexer;
use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let lexer = IndentLexer::new(&s);
    for token in lexer {
        println!("{:?}", token);
    }
}
