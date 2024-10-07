use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum TokenType {
    NUMBER,
    STRING,
    PAREN,
    BINARY_OPERATOR,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub val: String,
}

fn token(token_type: TokenType, val: String) -> Token {
    return Token { token_type, val };
}

pub fn tokenize(source_string: String) -> VecDeque<Token> {
    let mut cursor = 0;
    let mut token_array: VecDeque<Token> = VecDeque::new();

    while cursor < source_string.len() {
        let match_number = Regex::new(r"^\d+\.?\d+").unwrap();
        let match_field_string = Regex::new(r"^[a-zA-Z0-9._]+").unwrap();
        let match_string = Regex::new(r#"^"[^"]*""#).unwrap();
        let match_binary_operator =
            Regex::new(r"^(^(!=)|^(<=)|^(>=)|^(=)|^(<)|^(>)|^(\&\&)|^(\|\|)|^(\+)|^(\-))").unwrap();
        let match_paren = Regex::new(r"^[\(\)]").unwrap();

        if match_number.is_match(&source_string[cursor..]) {
            let value = match_number
                .find(&source_string[cursor..])
                .map(|x| x.as_str())
                .unwrap();
            cursor += value.len();
            token_array.push_back(token(TokenType::NUMBER, value.to_string()));
        } else if match_field_string.is_match(&source_string[cursor..]) {
            let value = match_field_string
                .find(&source_string[cursor..])
                .map(|x| x.as_str())
                .unwrap();
            cursor += value.len();
            token_array.push_back(token(TokenType::STRING, value.to_string()));
        } else if match_string.is_match(&source_string[cursor..]) {
            let value = match_string
                .find(&source_string[cursor..])
                .map(|x| x.as_str())
                .unwrap();
            cursor += value.len();
            token_array.push_back(token(TokenType::STRING, value.to_string()));
        } else if match_binary_operator.is_match(&source_string[cursor..]) {
            let value = match_binary_operator
                .find(&source_string[cursor..])
                .map(|x| x.as_str())
                .unwrap();
            cursor += value.len();
            token_array.push_back(token(TokenType::BINARY_OPERATOR, value.to_string()));
        } else if match_paren.is_match(&source_string[cursor..]) {
            let value = match_paren
                .find(&source_string[cursor..])
                .map(|x| x.as_str())
                .unwrap();
            cursor += value.len();
            token_array.push_back(token(TokenType::PAREN, value.to_string()));
        } else {
            cursor += 1;
        }
    }
    return token_array;
}
