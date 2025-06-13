use std::num::ParseFloatError;
use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
enum LexingError {
    InvalidNumber(String),

    #[default]
    UnrecognisedCharacter
}

impl From<ParseFloatError> for LexingError {
    fn from(_: ParseFloatError) -> Self {
        LexingError::InvalidNumber("Invalid number: {}".to_owned())
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", error = LexingError)]
enum Token {
    // Literals
    #[regex(r"([[:alpha:]]|_)[[:word:]]*'*", |lex| lex.slice().to_owned())]
    #[regex(r#"[^[[:digit:]][[:alpha:]][ \t\n\f]!'"_\(\)\[\],;:|\\]+"#, |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_owned())]
    String(String),

    #[regex(r"[[:digit:]]+(\.[[:digit:]]+)?([eE][-+]?[[:digit:]]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Num(f64),

    // Punctuation
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LSquare,

    #[token("]")]
    RSquare,

    #[token(",")]
    Comma,

    #[token(";")]
    SemiColon,

    // Reserved
    #[token("!")]
    Bang,

    #[token("++")]
    PlusPlus,

    #[token("---")]
    TripleDash,

    #[token(":")]
    Colon,

    #[token("<=")]
    LeftArrowFat, // IS

    #[token("==")]
    EqEq,

    #[token("=>")]
    RightArrowFat, // GIVES

    #[token("|")]
    Pipe,

    #[token("abstype")]
    AbsType,

    #[token("data")]
    Data,

    #[token("dec")]
    Dec,

    #[token("display")]
    Display,

    #[token("else")]
    Else,

    #[token("edit")]
    Edit,

    #[token("exit")]
    Exit,

    #[token("if")]
    If,

    #[token("in")]
    In,

    #[token("infix")]
    Infix,
    
    #[token("infixr")]
    #[token("infixrl")]
    InfixR,

    #[token("lambda")]
    #[token("\\")]
    Lambda,

    #[token("let")]
    Let,

    #[token("letrec")]
    LetRec,

    #[token("private")]
    Private,

    #[token("save")]
    Save,

    #[token("then")]
    Then,

    #[token("type")]
    Type,

    #[token("typevar")]
    TypeVar,

    #[token("use")]
    #[token("uses")]
    Uses,

    #[token("where")]
    Where,

    #[token("whererec")]
    WhereRec,

    #[token("write")]
    Write,

    // Compat
    #[token("end")]
    End,

    #[token("module")]
    Module,

    #[token("nonop")]
    NonOp,

    #[token("pubconst")]
    PubConst,

    #[token("pubfun")]
    PubFun,

    #[token("pubtype")]
    PubType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_lex_word_identifier() {
        let mut lex = Token::lexer("_lift0' pipe _hello say_world");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("_lift0'".to_owned()))));
        assert_eq!(lex.slice(), "_lift0'");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("pipe".to_owned()))));
        assert_eq!(lex.slice(), "pipe");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("_hello".to_owned()))));
        assert_eq!(lex.slice(), "_hello");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("say_world".to_owned()))));
        assert_eq!(lex.slice(), "say_world");
    }

    #[test]
    fn should_lex_non_word_identifier() {
        let mut lex = Token::lexer("# -> >> <> = /=");
        
        assert_eq!(lex.next(), Some(Ok(Token::Identifier("#".to_owned()))));
        assert_eq!(lex.slice(), "#");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("->".to_owned()))));
        assert_eq!(lex.slice(), "->");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier(">>".to_owned()))));
        assert_eq!(lex.slice(), ">>");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("<>".to_owned()))));
        assert_eq!(lex.slice(), "<>");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("=".to_owned()))));
        assert_eq!(lex.slice(), "=");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("/=".to_owned()))));
        assert_eq!(lex.slice(), "/=");
    }
    
    #[test]
    fn should_lex_reserved_and_identifier() {
        let mut lex = Token::lexer("++ +");

        assert_eq!(lex.next(), Some(Ok(Token::PlusPlus)));
        assert_eq!(lex.slice(), "++");

        assert_eq!(lex.next(), Some(Ok(Token::Identifier("+".to_owned()))));
        assert_eq!(lex.slice(), "+");
    }
    
    #[test]
    fn should_lex_aliases_correctly() {
        let mut lex = Token::lexer("\\ lambda use uses infixr infixrl");

        assert_eq!(lex.next(), Some(Ok(Token::Lambda)));
        assert_eq!(lex.slice(), "\\");

        assert_eq!(lex.next(), Some(Ok(Token::Lambda)));
        assert_eq!(lex.slice(), "lambda");

        assert_eq!(lex.next(), Some(Ok(Token::Uses)));
        assert_eq!(lex.slice(), "use");

        assert_eq!(lex.next(), Some(Ok(Token::Uses)));
        assert_eq!(lex.slice(), "uses");

        assert_eq!(lex.next(), Some(Ok(Token::InfixR)));
        assert_eq!(lex.slice(), "infixr");

        assert_eq!(lex.next(), Some(Ok(Token::InfixR)));
        assert_eq!(lex.slice(), "infixrl");
    }
}
