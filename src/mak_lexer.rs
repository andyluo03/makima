use crate::mak_tokens::*;

#[derive(Debug, Clone)]
pub struct Lexer {
    pub tokens_: Vec<Token>,
    pub state_: usize
}

impl Lexer {
    pub fn from_tokens(tokens: Vec<Token>) -> Lexer {
        return Lexer{tokens_:tokens, state_: 0};
    }

    pub fn from_string(input: &String) -> Lexer {
        let mut buffer = String::new();
        let mut tokens: Vec<Token> = Vec::new();
        
        /*
        Operators:
        +, -, *, /, =, ., (, ), {, }, ;

        Keywords:
        let, fn, if, else, elif, return, while
        */

        let key_chars = ['+', '-', '*', '/', '=', '.', '(', ')', '{', '}', ';', ',', '=', '<', '>'];
        let as_chars:Vec<_> = input.chars().collect();

        let mut read_ahead = false;
        for idx in 0..as_chars.len() {
            //Slow!
            if read_ahead { 
                read_ahead = false;
                continue;
            }

            let i = as_chars[idx];

            if i == ' ' || i == '\n' || key_chars.contains(&i) {
                if buffer.len() > 0 {
                    tokens.push(Token::new(buffer.clone())); //could try to avoid a clone with a move...
                }

                buffer.clear();

                if key_chars.contains(&i) {
                    match i {
                        '=' | '<' | '>' => {
                            if idx + 1 < as_chars.len() && as_chars[idx+1]== '=' {
                                let mut combined_operator = i.to_string();
                                combined_operator.push('=');
                                tokens.push(Token::new(combined_operator));

                                 read_ahead = true;
                            } else {
                                tokens.push(Token::new(i.to_string()));
                            }
                        }
                        _ => {tokens.push(Token::new(i.to_string()));}
                    }
                }
                continue;
            }

            buffer.push(i.clone());
        }

        if buffer.len() > 0 {
            tokens.push(Token::new(buffer.clone())); //could try to avoid a clone...
        }

        return Lexer{tokens_: tokens, state_: 0};
    }

    pub fn next(&mut self) -> Token {
        self.state_ += 1;
        return self.peek();
    }

    pub fn peek(&self) -> Token {
        return if self.state_ >= self.tokens_.len() { Token {token_type_: TokenType::EoF, contents_: "".to_string()} } else { self.tokens_[self.state_].clone() };
    }
}