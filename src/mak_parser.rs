
use std::any::Any;

use crate::mak_control_flow::*;
use crate::mak_lexer::*;
use crate::mak_tokens::*;
use crate::mak_basic_block::*;

/*
1: Parse functions into Vec<Tokens>
2: For each function, parse the if, while, elif, else blocks (this will create a "control flow graph" for each function)
3: Turn remaning tokens in blocks into Vec<Vec<Tokens>>
4: For each Vec<Token> convert into AST using Pratt Parser
5: Return CFGs for each function
6: Pass into type checker/code-generation stages/etc.
*/

pub struct Parser {

}

//Intermediate building blocks
#[derive(Clone, Debug)]
pub struct FunctionTokenBlock {
    pub name_: String,
    pub arguments_: Vec<Token>, //this will be parsed... later!
    pub return_: Vec<Token>,
    pub contents_: Vec<Token>,
}

impl FunctionTokenBlock {
    fn new() -> Self{
        return FunctionTokenBlock{name_:"".to_string(), arguments_: vec![], return_: vec![], contents_: vec![]};
    }

    fn set_name(&mut self, name: &String){
        self.name_ = name.clone();
    }

    fn add_token(&mut self, token: &Token){
        self.contents_.push(token.clone());
    }

    fn add_argument(&mut self, token: &Token){
        self.arguments_.push(token.clone());
    }

    fn add_return(&mut self, token: &Token){
        self.return_.push(token.clone());
    }
}

impl Parser {
    pub fn from_tokens(lexer: &Lexer) -> ControlFlowNode {
        /*
        Brackets define blocks -- { basic block }
        fn <name> (<arg1>: <type1>, <arg2>: <type2>) -> result {} //functions CANNOT be defined in functions -- might implement lambdas... NEVER!
        while ( condition ) {}
        if ( condition ) {}
        elif (condition) {}
        else {}

        NOT IMPLEMENTED YET!!!
         */
        let tokens = lexer.tokens_.clone();



        let dummy = ControlFlowNode{block_: BasicBlock{ expressions_: vec![] }, branches_: vec![]};
        return dummy;
    }

    pub fn get_functions(tokens: Vec<Token>) -> Vec<FunctionTokenBlock> {
        #[derive(Debug)]
        enum FunctionParseState {
            OutsideFunction,
            FunctionName, //ends on (, but if there are multiple tokens :<
            FunctionArguments, //ends on ), peek next -- if not "->" then except.
            FunctionReturn, //ends on "{"
            FunctionInterior, //ends on } AND unmatched_left_brackets = 0 and goes back to OutsideFunction
        }

        let mut unmatched_left_brackets = 0;
        let mut state = FunctionParseState::OutsideFunction;
        let mut idx = 0;

        let mut functions: Vec<FunctionTokenBlock> = Vec::new();
        let mut current_function = FunctionTokenBlock::new();

        while idx < tokens.len() {
            let current_token = tokens[idx].clone(); //unnecessary copy!
            match state {
                FunctionParseState::OutsideFunction => {
                    if current_token.contents_ != "fn" {
                        panic!("Line: {}: Expected {} but found {}", current_token.line_, "fn", tokens[idx].contents_);
                    }
                    
                    current_function = FunctionTokenBlock::new();
                    state = FunctionParseState::FunctionName;
                    idx += 1;
                },

                FunctionParseState::FunctionName => {
                    if tokens[idx].token_type_ != TokenType::Atom {
                        panic!("Line: {}: Expected function name but got: {}", current_token.line_, tokens[idx].contents_);
                    }

                    if idx + 1 >= tokens.len() {
                        panic!("Line: {}: Expected function contents but file ended.", 0);
                    }

                    if tokens[idx + 1].contents_ != "(" && tokens[idx + 1].token_type_ != TokenType::Operator {
                        panic!("Line: {}: Expected ( but found {}", current_token.line_, tokens[idx+1].contents_);
                    }

                    current_function.set_name(&tokens[idx].contents_);
                    idx = idx + 2;
                    state = FunctionParseState::FunctionArguments;
                },

                FunctionParseState::FunctionArguments => {
                    if current_token.contents_ == ")" {
                        if idx + 1 >= tokens.len() {
                            panic!("Line: {}: Expected function contents but file ended.", current_token.line_);
                        }

                        if tokens[idx + 1].contents_ != "->" {
                            panic!("Line: {}: Expected -> but found {}", current_token.line_, tokens[idx+1].contents_);
                        }

                        state = FunctionParseState::FunctionReturn;
                        idx = idx + 2;
                        continue;
                    }

                    current_function.add_argument(&current_token);
                    idx = idx + 1;
                },

                FunctionParseState::FunctionReturn => {
                    if idx + 1 >= tokens.len() {
                        panic!("File ends with function declaration {} incomplete.", current_function.name_);
                    }

                    if current_token.contents_ == "{" {
                        state = FunctionParseState::FunctionInterior;
                        idx = idx+1;
                        continue;
                    }

                    current_function.add_return(&current_token);
                    idx = idx+1;
                },

                FunctionParseState::FunctionInterior => {
                    if tokens[idx].contents_ == "}" && unmatched_left_brackets == 0 {
                        functions.append(&mut vec![current_function.clone()]);
                        state = FunctionParseState::OutsideFunction;
                        idx = idx + 1;
                        continue;
                    }

                    if current_token.contents_ == "{" && current_token.token_type_ == TokenType::Operator {
                        unmatched_left_brackets = unmatched_left_brackets + 1;
                    }

                    if current_token.contents_ == "}" && current_token.token_type_ == TokenType::Operator {
                        unmatched_left_brackets = unmatched_left_brackets - 1;
                    }

                    current_function.add_token(&current_token);
                    idx = idx+1;
                }
            }
        }

        return functions;
    }
}