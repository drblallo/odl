fn main() {
    use odl::lexer::IndentLexer;
    use std::io::Read;

    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let lexer = IndentLexer::new(&s);
    for token in lexer {
        println!("{:?}", token);
    }
}
