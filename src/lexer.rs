use std::fmt::{Display, Formatter};
use std::str::{FromStr, Lines};
use std::str::Chars;

pub struct Lexer<'a> {
    lines: Lines<'a>,
    line: Chars<'a>,
    row: usize,
    col: usize,
}
impl<'a> Lexer<'a> {
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
                    let mut buf = [0; 4];
                    let next_char_str = next_char.encode_utf8(&mut buf);
                    if let Ok(token_type) = TokenType::from_str(next_char_str) {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub row: usize,
    pub col: usize,
}
impl Token {
    pub fn new(token_type: TokenType, row: usize, col: usize) -> Self {
        Self { token_type, row, col }
    }
}
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
impl FromStr for TokenType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "," => Ok(Self::Input),
            "." => Ok(Self::Output),
            "<" => Ok(Self::MoveLeft),
            ">" => Ok(Self::MoveRight),
            "+" => Ok(Self::Increment),
            "-" => Ok(Self::Decrement),
            "[" => Ok(Self::BracketLeft),
            "]" => Ok(Self::BracketRight),
            _ => Err(()),
        }
    }
}
impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
               Self::Input => ",",
               Self::Output => ".",
               Self::MoveLeft => "<",
               Self::MoveRight => ">",
               Self::Increment => "+",
               Self::Decrement => "-",
               Self::BracketLeft => "[",
               Self::BracketRight => "]",
            }
        )
    }
}
