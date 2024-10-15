use regex::Regex;
use std::collections::VecDeque;
use std::sync::LazyLock;

#[derive(Debug)]
pub enum TokenType {
    Number,
    String,
    Paren,
    Binary,
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
    Regex::new(r"^(^(!=)|^(<=)|^(>=)|^(=)|^(<)|^(>)|^(&&)|^(\|\|)|^(\+)|^(-))").unwrap()
});

static MATCH_PAREN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[()]").unwrap());

fn token(token_type: TokenType, val: String) -> Token {
    Token { token_type, val }
}

pub fn tokenize(source_string: &str) -> VecDeque<Token> {
    let mut cursor = 0;
    let mut token_array: VecDeque<Token> = VecDeque::new();
    while cursor < source_string.len() {
        if let Some(val) = MATCH_NUMBER
            .find(&source_string[cursor..])
            .map(|x| x.as_str())
        {
            cursor += val.len();
            token_array.push_back(token(TokenType::Number, val.to_string()));
        } else if let Some(val) = MATCH_FIELD_STRING
            .find(&source_string[cursor..])
            .map(|x| x.as_str())
        {
            cursor += val.len();
            token_array.push_back(token(TokenType::String, val.to_string()));
        } else if let Some(val) = MATCH_STRING
            .find(&source_string[cursor..])
            .map(|x| x.as_str().replace("'", "\""))
        {
            cursor += val.len();
            token_array.push_back(token(TokenType::String, val));
        } else if let Some(val) = MATCH_BINARY_OPERATOR
            .find(&source_string[cursor..])
            .map(|x| x.as_str())
        {
            cursor += val.len();
            token_array.push_back(token(TokenType::Binary, val.to_string()));
        } else if let Some(val) = MATCH_PAREN
            .find(&source_string[cursor..])
            .map(|x| x.as_str())
        {
            cursor += val.len();
            token_array.push_back(token(TokenType::Paren, val.to_string()));
        } else {
            cursor += 1;
        }
    }
    token_array
}
