#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    WhiteSpace,
    Unknown,
    EOF,
}

#[derive(Debug)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            self.current_pos += 1;
            let eof_char: char = '\0';

            return Some(Token::new(
                TokenKind::EOF,
                TextSpan {
                    start: 0,
                    end: 0,
                    literal: eof_char.to_string(),
                },
            ));
        }

        let c = self.current_char();

        return c.map(|c| {
            let start = self.current_pos;
            let mut kind = TokenKind::Unknown;

            if Self::is_num_start(&c) {
                let number: i64 = self.consume_number();
                kind = TokenKind::Number(number);
            } else if Self::is_whitespace(&c) {
                self.consume();
                kind = TokenKind::WhiteSpace;
            } else {
                kind = self.consume_punctiation();
            }

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);

            Token::new(kind, span)
        });
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn is_num_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;

        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }

        number
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn consume_punctiation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => TokenKind::Unknown,
        }
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }

        let c = self.current_char();
        self.current_pos += 1;

        c
    }
}
