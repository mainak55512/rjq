use regex::Regex;
use std::collections::VecDeque;
use std::sync::LazyLock;

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

static MATCH_NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+\.?\d+").unwrap());

static MATCH_FIELD_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._]+").unwrap());

static MATCH_STRING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^'(.*?)'"#).unwrap());

static MATCH_BINARY_OPERATOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(^(!=)|^(<=)|^(>=)|^(=)|^(<)|^(>)|^(\&\&)|^(\|\|)|^(\+)|^(\-))").unwrap()
});

static MATCH_PAREN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[\(\)]").unwrap());

fn token(token_type: TokenType, val: String) -> Token {
    return Token { token_type, val };
}

pub fn tokenize(source_string: &str) -> VecDeque<Token> {
    let mut cursor = 0;
    let mut token_array: VecDeque<Token> = VecDeque::new();
    while cursor < source_string.len() {
        if MATCH_NUMBER.is_match(&source_string[cursor..]) {
            match MATCH_NUMBER
                .find(&source_string[cursor..])
                .map(|x| x.as_str())
            {
                Some(val) => {
                    cursor += val.len();
                    token_array.push_back(token(TokenType::NUMBER, val.to_string()));
                }
                None => (),
            }
        } else if MATCH_FIELD_STRING.is_match(&source_string[cursor..]) {
            match MATCH_FIELD_STRING
                .find(&source_string[cursor..])
                .map(|x| x.as_str())
            {
                Some(val) => {
                    cursor += val.len();
                    token_array.push_back(token(TokenType::STRING, val.to_string()));
                }
                None => (),
            };
        } else if MATCH_STRING.is_match(&source_string[cursor..]) {
            match MATCH_STRING
                .find(&source_string[cursor..])
                .map(|x| x.as_str().replace("'", "\""))
            {
                Some(val) => {
                    cursor += val.len();
                    token_array.push_back(token(TokenType::STRING, val));
                }
                None => (),
            };
        } else if MATCH_BINARY_OPERATOR.is_match(&source_string[cursor..]) {
            match MATCH_BINARY_OPERATOR
                .find(&source_string[cursor..])
                .map(|x| x.as_str())
            {
                Some(val) => {
                    cursor += val.len();
                    token_array.push_back(token(TokenType::BINARY_OPERATOR, val.to_string()));
                }
                None => (),
            };
        } else if MATCH_PAREN.is_match(&source_string[cursor..]) {
            match MATCH_PAREN
                .find(&source_string[cursor..])
                .map(|x| x.as_str())
            {
                Some(val) => {
                    cursor += val.len();
                    token_array.push_back(token(TokenType::PAREN, val.to_string()));
                }
                None => (),
            };
        } else {
            cursor += 1;
        }
    }
    return token_array;
}
