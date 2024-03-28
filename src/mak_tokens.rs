#[derive(Clone, Copy)]

/*
Operators:
+, -, *, /, =, ., (, ), {, }

Keywords:
let, fn, if, else, elif, return, while

Semicolon:
;
*/

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Operator, //differentiating unary vs binary operators might require distinction here!
    Keyword,
    Atom,
    EoF
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type_: TokenType,
    pub contents_: String,
    pub line_: usize
}

impl Token {
    pub fn new(contents: String, line: usize) -> Self{
        let slice = &contents[0..contents.len()];

        //Floats may be difficult -- will need to lookahead for certain expressions
        let operators = ["+", "-", "*", "/", "=", ".", "(", ")", "{", "}", ";", ",", "==", "<=", ">=", "<", ">", "->"];
        let keywords = ["let", "fn", "if", "else", "elif", "return", "while"];

        let token_type: TokenType;
        match slice {
            x if operators.contains(&x) => { token_type = TokenType::Operator},
            x if keywords.contains(&x) => {token_type = TokenType::Keyword},
            _ => { token_type = TokenType::Atom; },
        }

        return Token{token_type_:token_type, contents_:contents.clone(), line_: line};
    }
}
