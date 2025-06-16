use std::fs;
use logos::Logos;
use hope::syntax::token::Token;

fn main() {
    let file_path = "./lib/Standard.hop";
    let contents = fs::read_to_string(file_path)
        .expect("Should be able to read file");

    let lex = Token::lexer_with_extras(&contents, 1);

    for tok in lex {
        match tok {
            Ok(token) => println!("{:?}", token),
            Err(e) => eprintln!("{:#?}", e)
        }
    }
}
