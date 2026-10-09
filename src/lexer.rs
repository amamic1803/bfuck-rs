//! A lexer for the Brainfuck programming language.

use std::fmt::{Debug, Display, Formatter};
use std::str::Lines;
use std::str::Chars;
use thiserror::Error;

/// An error that can occur during lexing.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Error)]
pub enum LexerError {
    #[error("character doesn't correspond to any token type")]
    InvalidCharacter,
}

/// A lexer for the Brainfuck programming language.
///
/// Implements [Iterator] to produce [Token]s from source code.
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    lines: Lines<'a>,
    line: Chars<'a>,
    row: usize,
    col: usize,
}
impl<'a> Lexer<'a> {
    /// Creates a new [Lexer] from the given source code string.
    /// # Arguments
    /// * `input` - The source code string to lex.
    /// # Returns
    /// * A new [Lexer] that will produce [Token]s from the given source code.
    pub fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines(),
            line: "".chars(),
            row: 0,
            col: 0,
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.line.next() {
                Some(next_char) => {
                    self.col += 1;
                    if let Ok(token_type) = TokenType::try_from(next_char) {
                        return Some(Token::new(token_type, self.row, self.col));
                    }
                }
                None => {
                    match self.lines.next() {
                        Some(next_line) => {
                            self.row += 1;
                            self.line = next_line.chars();
                            self.col = 0;
                        }
                        None => return None,
                    }
                },
            }
        }
    }
}

/// A token produced by the lexer.
///
/// Row and column are 1-indexed, meaning that the first row and column are `1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    token_type: TokenType,
    row: usize,
    col: usize,
}
impl Token {
    /// Creates a new [Token] with the given type, row, and column.
    /// # Arguments
    /// * `token_type` - The type of the token.
    /// * `row` - The row of the token in the source code.
    /// * `col` - The column of the token in the source code.
    /// # Returns
    /// A new [Token] with the given type, row, and column.
    pub fn new(token_type: TokenType, row: usize, col: usize) -> Self {
        Self { token_type, row, col }
    }
    /// The type of the token.
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
    /// The row of the token in the source code.
    pub fn row(&self) -> usize {
        self.row
    }
    /// The column of the token in the source code.
    pub fn col(&self) -> usize {
        self.col
    }
}

/// A type of token produced by the lexer.
///
/// Each token type corresponds to a specific character in the Brainfuck language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Input,
    Output,
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    BracketLeft,
    BracketRight,
}
impl TryFrom<char> for TokenType {
    type Error = LexerError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ',' => Ok(Self::Input),
            '.' => Ok(Self::Output),
            '<' => Ok(Self::MoveLeft),
            '>' => Ok(Self::MoveRight),
            '+' => Ok(Self::Increment),
            '-' => Ok(Self::Decrement),
            '[' => Ok(Self::BracketLeft),
            ']' => Ok(Self::BracketRight),
            _ => Err(LexerError::InvalidCharacter),
        }
    }
}
impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
               Self::Input => ',',
               Self::Output => '.',
               Self::MoveLeft => '<',
               Self::MoveRight => '>',
               Self::Increment => '+',
               Self::Decrement => '-',
               Self::BracketLeft => '[',
               Self::BracketRight => ']',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_new_and_accessors() {
        let token = Token::new(TokenType::Increment, 3, 7);

        assert_eq!(token.token_type(), TokenType::Increment);
        assert_eq!(token.row(), 3);
        assert_eq!(token.col(), 7);
    }

    #[test]
    fn token_type_maps_brainfuck_characters() {
        let cases = [
            (',', TokenType::Input),
            ('.', TokenType::Output),
            ('<', TokenType::MoveLeft),
            ('>', TokenType::MoveRight),
            ('+', TokenType::Increment),
            ('-', TokenType::Decrement),
            ('[', TokenType::BracketLeft),
            (']', TokenType::BracketRight),
        ];

        for (character, expected) in cases {
            assert_eq!(TokenType::try_from(character), Ok(expected));
        }
    }

    #[test]
    fn token_type_rejects_invalid_characters() {
        for character in ['a', ' ', '\n', '\0', '😊'] {
            assert_eq!(
                TokenType::try_from(character),
                Err(LexerError::InvalidCharacter)
            );
        }
    }

    #[test]
    fn token_type_displays_brainfuck_characters() {
        let cases = [
            (TokenType::Input, ','),
            (TokenType::Output, '.'),
            (TokenType::MoveLeft, '<'),
            (TokenType::MoveRight, '>'),
            (TokenType::Increment, '+'),
            (TokenType::Decrement, '-'),
            (TokenType::BracketLeft, '['),
            (TokenType::BracketRight, ']'),
        ];

        for (token_type, expected) in cases {
            assert_eq!(token_type.to_string(), expected.to_string());
        }
    }

    #[test]
    fn lexer_returns_tokens_with_one_indexed_positions() {
        let tokens: Vec<_> = Lexer::new(" +>-\n[.] ").collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Increment, 1, 2),
                Token::new(TokenType::MoveRight, 1, 3),
                Token::new(TokenType::Decrement, 1, 4),
                Token::new(TokenType::BracketLeft, 2, 1),
                Token::new(TokenType::Output, 2, 2),
                Token::new(TokenType::BracketRight, 2, 3),
            ]
        );
    }

    #[test]
    fn lexer_ignores_non_brainfuck_characters() {
        let tokens: Vec<_> = Lexer::new("comment + code").collect();

        assert_eq!(
            tokens,
            vec![Token::new(TokenType::Increment, 1, 9)]
        );
    }

    #[test]
    fn lexer_handles_empty_and_whitespace_only_input() {
        assert_eq!(Lexer::new("").next(), None);
        assert_eq!(Lexer::new(" \n\t\r\n").collect::<Vec<_>>(), Vec::new());
    }

    #[test]
    fn lexer_continues_after_empty_lines() {
        let tokens: Vec<_> = Lexer::new("\n>\n\n+").collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::MoveRight, 2, 1),
                Token::new(TokenType::Increment, 4, 1),
            ]
        );
    }
}
