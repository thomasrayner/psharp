use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Identifier(String),
    True,
    False,
    Null,

    // Keywords
    Let,
    Fn,
    If,
    Else,
    For,
    While,
    Break,
    Continue,
    Return,
    Match,
    Use,
    Def,
    Async,
    Await,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    Dot,
    DoubleColon,
    Arrow,
    FatArrow,
    PipeOp,
    Question,
    Ampersand,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,

    // Special
    Eof,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    while let Some(&ch) = chars.peek() {
        match ch {
            // Whitespace
            ' ' | '\t' => {
                chars.next();
                column += 1;
            }
            '\n' => {
                tokens.push(Token {
                    token_type: TokenType::Newline,
                    line,
                    column,
                });
                chars.next();
                line += 1;
                column = 1;
            }
            '\r' => {
                chars.next();
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                line += 1;
                column = 1;
            }

            // Comments
            '#' => {
                chars.next();
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
            }

            // Strings
            '"' => {
                chars.next();
                let start_col = column;
                let mut string = String::new();
                column += 1;

                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        column += 1;
                        break;
                    } else if c == '\\' {
                        chars.next();
                        column += 1;
                        if let Some(&next) = chars.peek() {
                            chars.next();
                            column += 1;
                            match next {
                                'n' => string.push('\n'),
                                't' => string.push('\t'),
                                'r' => string.push('\r'),
                                '\\' => string.push('\\'),
                                '"' => string.push('"'),
                                _ => {
                                    string.push('\\');
                                    string.push(next);
                                }
                            }
                        }
                    } else {
                        chars.next();
                        column += 1;
                        string.push(c);
                    }
                }

                tokens.push(Token {
                    token_type: TokenType::String(string),
                    line,
                    column: start_col,
                });
            }

            '\'' => {
                chars.next();
                let start_col = column;
                let mut string = String::new();
                column += 1;

                while let Some(&c) = chars.peek() {
                    if c == '\'' {
                        chars.next();
                        column += 1;
                        break;
                    } else {
                        chars.next();
                        column += 1;
                        string.push(c);
                    }
                }

                tokens.push(Token {
                    token_type: TokenType::String(string),
                    line,
                    column: start_col,
                });
            }

            // Numbers
            '0'..='9' => {
                let start_col = column;
                let mut number = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_numeric() || c == '.' {
                        number.push(c);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                let num: f64 = number.parse()?;
                tokens.push(Token {
                    token_type: TokenType::Number(num),
                    line,
                    column: start_col,
                });
            }

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                let start_col = column;
                let mut ident = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' || c == '?' || c == '!' {
                        ident.push(c);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                let token_type = match ident.as_str() {
                    "let" => TokenType::Let,
                    "fn" => TokenType::Fn,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "for" => TokenType::For,
                    "while" => TokenType::While,
                    "break" => TokenType::Break,
                    "continue" => TokenType::Continue,
                    "return" => TokenType::Return,
                    "match" => TokenType::Match,
                    "use" => TokenType::Use,
                    "def" => TokenType::Def,
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    "null" => TokenType::Null,
                    "and" => TokenType::And,
                    "or" => TokenType::Or,
                    "not" => TokenType::Not,
                    "async" => TokenType::Async,
                    "await" => TokenType::Await,
                    _ => TokenType::Identifier(ident),
                };

                tokens.push(Token {
                    token_type,
                    line,
                    column: start_col,
                });
            }

            // Operators and delimiters
            '+' => {
                tokens.push(Token {
                    token_type: TokenType::Plus,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '-' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&'>') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::Arrow,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Minus,
                        line,
                        column: column - 1,
                    });
                }
            }
            '*' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&'*') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::Power,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Star,
                        line,
                        column: column - 1,
                    });
                }
            }
            '/' => {
                tokens.push(Token {
                    token_type: TokenType::Slash,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '%' => {
                tokens.push(Token {
                    token_type: TokenType::Percent,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '=' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::EqualEqual,
                        line,
                        column: column - 2,
                    });
                } else if chars.peek() == Some(&'>') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::FatArrow,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Equal,
                        line,
                        column: column - 1,
                    });
                }
            }
            '!' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::NotEqual,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Not,
                        line,
                        column: column - 1,
                    });
                }
            }
            '<' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::LessEqual,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Less,
                        line,
                        column: column - 1,
                    });
                }
            }
            '>' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::GreaterEqual,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Greater,
                        line,
                        column: column - 1,
                    });
                }
            }
            '&' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&'&') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::And,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Ampersand,
                        line,
                        column: column - 1,
                    });
                }
            }
            '|' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&'|') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::Or,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::PipeOp,
                        line,
                        column: column - 1,
                    });
                }
            }
            '.' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&'.') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::DoubleColon,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Dot,
                        line,
                        column: column - 1,
                    });
                }
            }
            ':' => {
                chars.next();
                column += 1;
                if chars.peek() == Some(&':') {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        token_type: TokenType::DoubleColon,
                        line,
                        column: column - 2,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Colon,
                        line,
                        column: column - 1,
                    });
                }
            }
            '?' => {
                tokens.push(Token {
                    token_type: TokenType::Question,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::LeftParen,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::RightParen,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '{' => {
                tokens.push(Token {
                    token_type: TokenType::LeftBrace,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '}' => {
                tokens.push(Token {
                    token_type: TokenType::RightBrace,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '[' => {
                tokens.push(Token {
                    token_type: TokenType::LeftBracket,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            ']' => {
                tokens.push(Token {
                    token_type: TokenType::RightBracket,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            ',' => {
                tokens.push(Token {
                    token_type: TokenType::Comma,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }

            _ => {
                return Err(anyhow!("Unknown character '{}' at line {}, column {}", ch, line, column));
            }
        }
    }

    tokens.push(Token {
        token_type: TokenType::Eof,
        line,
        column,
    });

    Ok(tokens)
}
