//! todo parser, create grammar that automatically detects and optimizes certain patterns
//! Grammar:
//! S -> P
//! P -> epsilon
//! P -> C P
//! C -> L
//! L -> lb C rb
//! C -> inc | dec | in | out | ml | mr

use thiserror::Error;
use crate::lexer::Token;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParserError {
    #[error("character doesn't correspond to any token type")]
    UnexpectedToken(Token),
    #[error("character doesn't correspond to any token type")]
    UnexpectedEndOfInput,
}

fn parse<T>(token_stream: T)
where
    T: IntoIterator<Item = Token>,
{
    let mut state_stack: Vec<usize> = vec![0]; // Stack to hold states
    let mut ast_stack: Vec<ASTNode> = Vec::new(); // Stack to hold AST nodes

    let mut token_stream = token_stream.into_iter();

    loop {
        let current_state = *state_stack.last().unwrap();
        let next_token = token_stream.next(); // Get the next token

        let action = match next_token {
            Some(token) => {
                let token_index: usize = token.token_type().into();
                ACTION_TABLE[current_state][token_index]
            }
            None => Action::Accept, // If no more tokens, accept
        };

        match action {
            Action::Shift(new_state) => {
                state_stack.push(new_state);
                ast_stack.push(ASTNode { id: new_state, node_type: ASTNodeType::Command, children: Vec::new() });
            }
            Action::Reduce(rule_number) => {
                // Handle reduction based on the rule number
                // This is where you would pop from the state and AST stacks and create a new AST node
            }
            Action::Accept => {
                println!("Parsing completed successfully.");
                break;
            }
            Action::Error => {
                println!("Parsing error at state {}.", current_state);
                break;
            }
        }
    }
}


pub struct ASTNode {
    id: usize,
    node_type: ASTNodeType,
    children: Vec<ASTNode>,
}

pub enum ASTNodeType {
    Program,
    Command,
    Loop,
    Input,
    Output,
    Move,
    Addition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Shift(usize), // Shift to a new state
    Reduce(usize), // Reduce by a production rule
    Accept, // Accept the input
    Error, // Error in parsing
}
// lb, rb, inc, dec, in, out, ml, mr, $
pub static ACTION_TABLE: [[Action; 9]; 22] = [
    [Action::Shift(3), Action::Error, Action::Shift(4), Action::Shift(5), Action::Shift(6), Action::Shift(7), Action::Shift(8), Action::Shift(9), Action::Reduce(1)],
    [Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Accept],
    [Action::Shift(3), Action::Error, Action::Shift(4), Action::Shift(5), Action::Shift(6), Action::Shift(7), Action::Shift(8), Action::Shift(9), Action::Reduce(1)],
    [Action::Shift(12), Action::Error, Action::Shift(13), Action::Shift(14), Action::Shift(15), Action::Shift(16), Action::Shift(17), Action::Shift(18), Action::Error],
    [Action::Reduce(4), Action::Reduce(4), Action::Reduce(4), Action::Reduce(4), Action::Reduce(4), Action::Reduce(4), Action::Reduce(4), Action::Reduce(4), Action::Reduce(4)],
    [Action::Reduce(5), Action::Reduce(5), Action::Reduce(5), Action::Reduce(5), Action::Reduce(5), Action::Reduce(5), Action::Reduce(5), Action::Reduce(5), Action::Reduce(5)],
    [Action::Reduce(6), Action::Reduce(6), Action::Reduce(6), Action::Reduce(6), Action::Reduce(6), Action::Reduce(6), Action::Reduce(6), Action::Reduce(6), Action::Reduce(6)],
    [Action::Reduce(7), Action::Reduce(7), Action::Reduce(7), Action::Reduce(7), Action::Reduce(7), Action::Reduce(7), Action::Reduce(7), Action::Reduce(7), Action::Reduce(7)],
    [Action::Reduce(8), Action::Reduce(8), Action::Reduce(8), Action::Reduce(8), Action::Reduce(8), Action::Reduce(8), Action::Reduce(8), Action::Reduce(8), Action::Reduce(8)],
    [Action::Reduce(9), Action::Reduce(9), Action::Reduce(9), Action::Reduce(9), Action::Reduce(9), Action::Reduce(9), Action::Reduce(9), Action::Reduce(9), Action::Reduce(9)],
    [Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Reduce(2)],
    [Action::Error, Action::Shift(19), Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error],
    [Action::Shift(12), Action::Error, Action::Shift(13), Action::Shift(14), Action::Shift(15), Action::Shift(16), Action::Shift(17), Action::Shift(18), Action::Error],
    [Action::Error, Action::Reduce(4), Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error],
    [Action::Error, Action::Reduce(5), Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error],
    [Action::Error, Action::Reduce(6), Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error],
    [Action::Error, Action::Reduce(7), Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error],
    [Action::Error, Action::Reduce(8), Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error],
    [Action::Error, Action::Reduce(9), Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error],
    [Action::Reduce(3), Action::Error, Action::Reduce(3), Action::Reduce(3), Action::Reduce(3), Action::Reduce(3), Action::Reduce(3), Action::Reduce(3), Action::Reduce(3)],
    [Action::Error, Action::Shift(21), Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error],
    [Action::Error, Action::Reduce(3), Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error, Action::Error],
];

// S (xxx), P, C, L
pub static GOTO_TABLE: [[Option<usize>; 3]; 24] = [
    [Some(1), Some(2), Some(3)],
    [None, None, None],
    [Some(11), Some(2), Some(3)],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, Some(12), Some(13)],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, None, None],
    [None, Some(22), Some(13)],
    [None, None, None],
    [None, None, None],
    [None, None, None],
];
