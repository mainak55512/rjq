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

impl Token {
    fn new(token_type: TokenType, val: String) -> Self {
        Self { token_type, val }
    }
}

static MATCH_NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+\.?\d+").unwrap());
static MATCH_IDENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._]+").unwrap());
static MATCH_STRING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^'(.*?)'"#).unwrap());
static MATCH_BINARY_OPERATOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(^(!=)|^(<=)|^(>=)|^(=)|^(<)|^(>)|^(&&)|^(\|\|)|^(\+)|^(-))").unwrap()
});
static MATCH_PAREN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[()]").unwrap());

pub(super) fn tokenize(source_string: &str) -> VecDeque<Token> {
    let mut cursor = 0;
    let mut tokens = VecDeque::new();
    while cursor < source_string.len() {
        if let Some(val) = MATCH_NUMBER
            .find(&source_string[cursor..])
            .map(|x| x.as_str())
        {
            cursor += val.len();
            tokens.push_back(Token::new(TokenType::Number, val.to_string()));
        } else if let Some(val) = MATCH_IDENT
            .find(&source_string[cursor..])
            .map(|x| x.as_str())
        {
            cursor += val.len();
            tokens.push_back(Token::new(TokenType::String, val.to_string()));
        } else if let Some(cap) = MATCH_STRING.captures(&source_string[cursor..]) {
            let (full, [val]) = cap.extract();
            cursor += full.len();
            tokens.push_back(Token::new(TokenType::String, val.to_string()));
        } else if let Some(val) = MATCH_BINARY_OPERATOR
            .find(&source_string[cursor..])
            .map(|x| x.as_str())
        {
            cursor += val.len();
            tokens.push_back(Token::new(TokenType::Binary, val.to_string()));
        } else if let Some(val) = MATCH_PAREN
            .find(&source_string[cursor..])
            .map(|x| x.as_str())
        {
            cursor += val.len();
            tokens.push_back(Token::new(TokenType::Paren, val.to_string()));
        } else {
            cursor += 1;
        }
    }
    tokens
}
