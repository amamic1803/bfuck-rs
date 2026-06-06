pub struct Lexer {

}

pub struct Token {
    token_type: TokenType,
    line: usize,
    column: usize,
}

pub enum TokenType {
    Input,
    Output,
    MovL,
    MovR,
    Inc,
    Dec,
    Left,
    Right,
}
