use odl::error::ParserError;
use odl::expression::*;

fn exit_on_error<T>(arg: Result<T, ParserError>) -> T {
    return match arg {
        Ok(parser) => parser,
        Err(error) => {
            eprintln!("Error: {:#?}", error);
            std::process::exit(-1);
        }
    };
}

fn main() {
    use odl::parser::Parser;
    use std::io::Read;

    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let result = exit_on_error(Parser::parse(&s));
    println!("{}", result);
}
