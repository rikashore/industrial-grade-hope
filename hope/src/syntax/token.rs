use std::num::ParseFloatError;
use logos::{Lexer, Logos, Skip, Span};

#[derive(Debug, PartialEq)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
    pub range: Span,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidNumber(String),

    #[default]
    UnrecognisedCharacter
}

impl From<ParseFloatError> for LexingError {
    fn from(_: ParseFloatError) -> Self {
        LexingError::InvalidNumber("Invalid number: {}".to_owned())
    }
}

fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras += 1;
    Skip
}

fn string_callback(lex: &mut Lexer<Token>) -> (String, Pos) {
    let body = lex.slice().to_owned();
    let pos = Pos {
        line: lex.extras,
        column: lex.span().start + 1,
        range: lex.span()
    };

    (body, pos)
}

fn loc_callback(lex: &mut Lexer<Token>) -> Pos {
    Pos {
        line: lex.extras,
        column: lex.span().start + 1,
        range: lex.span()
    }
}

fn num_callback(lex: &mut Lexer<Token>) -> Result<(f64, Pos), LexingError> {
    let body = lex.slice().parse::<f64>();
    match body {
        Err(e) => Err(<Token as Logos>::Error::from(e)),
        Ok(n) => {
            let pos = Pos {
                line: lex.extras,
                column: lex.span().start + 1,
                range: lex.span()
            };
            Ok((n, pos))
        }
    }
}

// TODO: I think comment lexing might be broken, once again I should write better tests
// TODO: Number parsing is slightly broken, 4.a parses as "4.0", ".", and "a" which is wrong
//       It should be an error
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+", error = LexingError, extras = usize)]
pub enum Token {
    // Newline handling for positions
    #[regex(r"\n", newline_callback)]
    Newline,

    // Literals
    #[regex(r"([[:alpha:]]|_)[[:word:]]*'*", string_callback)]
    #[regex(r#"[^[[:digit:]][[:alpha:]][ \t\n\f]!'"_\(\)\[\],;:|\\]+"#, string_callback)]
    Identifier((String, Pos)),

    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, string_callback)]
    String((String, Pos)),

    #[regex(r"[[:digit:]]+(\.[[:digit:]]+)?([eE][-+]?[[:digit:]]+)?", num_callback)]
    Num((f64, Pos)),

    // Punctuation
    #[token("(", loc_callback)]
    LParen(Pos),

    #[token(")", loc_callback)]
    RParen(Pos),

    #[token("[", loc_callback)]
    LSquare(Pos),

    #[token("]", loc_callback)]
    RSquare(Pos),

    #[token(",", loc_callback)]
    Comma(Pos),

    #[token(";", loc_callback)]
    SemiColon(Pos),

    // Reserved
    #[token("!", loc_callback)]
    Bang(Pos),

    #[token("++", loc_callback)]
    PlusPlus(Pos),

    #[token("---", loc_callback)]
    TripleDash(Pos),

    #[token(":", loc_callback)]
    Colon(Pos),

    #[token("<=", loc_callback)]
    LeftArrowFat(Pos), // IS

    #[token("==", loc_callback)]
    EqEq(Pos),

    #[token("=>", loc_callback)]
    RightArrowFat(Pos), // GIVES

    #[token("|", loc_callback)]
    Pipe(Pos),

    #[token("abstype", loc_callback)]
    AbsType(Pos),

    #[token("data", loc_callback)]
    Data(Pos),

    #[token("dec", loc_callback)]
    Dec(Pos),

    #[token("display", loc_callback)]
    Display(Pos),

    #[token("else", loc_callback)]
    Else(Pos),

    #[token("edit", loc_callback)]
    Edit(Pos),

    #[token("exit", loc_callback)]
    Exit(Pos),

    #[token("if", loc_callback)]
    If(Pos),

    #[token("in", loc_callback)]
    In(Pos),

    #[token("infix", loc_callback)]
    Infix(Pos),
    
    #[token("infixr", loc_callback)]
    #[token("infixrl", loc_callback)]
    InfixR(Pos),

    #[token("lambda", loc_callback)]
    #[token("\\", loc_callback)]
    Lambda(Pos),

    #[token("let", loc_callback)]
    Let(Pos),

    #[token("letrec", loc_callback)]
    LetRec(Pos),

    #[token("private", loc_callback)]
    Private(Pos),

    #[token("save", loc_callback)]
    Save(Pos),

    #[token("then", loc_callback)]
    Then(Pos),

    #[token("type", loc_callback)]
    Type(Pos),

    #[token("typevar", loc_callback)]
    TypeVar(Pos),

    #[token("use", loc_callback)]
    #[token("uses", loc_callback)]
    Uses(Pos),

    #[token("where", loc_callback)]
    Where(Pos),

    #[token("whererec", loc_callback)]
    WhereRec(Pos),

    #[token("write", loc_callback)]
    Write(Pos),

    // Compat
    #[token("end", loc_callback)]
    End(Pos),

    #[token("module", loc_callback)]
    Module(Pos),

    #[token("nonop", loc_callback)]
    NonOp(Pos),

    #[token("pubconst", loc_callback)]
    PubConst(Pos),

    #[token("pubfun", loc_callback)]
    PubFun(Pos),

    #[token("pubtype", loc_callback)]
    PubType(Pos),
}

#[cfg(test)]
mod tests {
    // TODO: Update tests and create proper testing method
    
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn should_lex_word_identifier() {
        let identifiers = HashMap::from([
            ("_lift0'", Pos {
                line: 1,
                column: 1,
                range: 0..7
            }),
        ]);

        let mut lex = Token::lexer_with_extras("_lift0'", 1);

        while let Some(tok) = lex.next() {
            if let Ok(Token::Identifier((name, pos))) = tok {
                assert_eq!(lex.slice(), name);
                let det_pos = identifiers.get(&name.as_str());
                assert_eq!(det_pos, Some(&pos))
            }
        }
    }
}
