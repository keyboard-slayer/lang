use crate::tokens::Token;

pub struct Lexer {
    input: String,
    position: usize,
    lineno: usize,
    offset: usize,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Self {
            input: s,
            position: 0,
            lineno: 0,
            offset: 0,
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut token = String::new();

        for ch in self.input.chars().skip(self.position - 1) {
            match ch {
                'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => token.push(ch),
                _ => break,
            }
        }

        self.position += token.len() - 1;

        match token.as_str() {
            "fn" => Token::Fn,
            "import" => Token::Import,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(token),
        }
    }

    fn read_str(&mut self) -> Token {
        let mut s = String::new();

        for ch in self.input.chars().skip(self.position) {
            if ch == '\"' {
                break;
            }

            s.push(ch);
        }

        self.position += s.len() + 1;

        Token::Str(s)
    }

    fn next_with_eq(&mut self, ch: char) -> Token {
        if let Some(eq) = self.input.chars().nth(self.position) {
            self.position += 1;
            
            if eq == '=' {
                return match ch {
                    '+' => Token::PlusEq,
                    '-' => Token::MinusEq,
                    '*' => Token::AsterixEq,
                    '/' => Token::SlashEq,
                    '%' => Token::ModEq,
                    '>' => Token::GtEq,
                    '<' => Token::LtEq,
                    '=' => Token::EqEq,
                    '!' => Token::BangEq,
                    _ => Token::Illegal(self.position, self.offset, self.lineno),
                }
            }
        }

        match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterix,
            '/' => Token::Slash,
            '%' => Token::Mod,
            '>' => Token::Gt,
            '<' => Token::Lt,
            '=' => Token::Eq,
            '!' => Token::Bang,
             _ => Token::Illegal(self.position, self.offset, self.lineno),
        }

    }

    pub fn read_number(&mut self) -> Token {
        let mut token = String::new();

        for ch in self.input.chars().skip(self.position - 1) {
            match ch {
                '0'..='9' => token.push(ch),
                _ => break,
            }
        }

        self.position += token.len() - 1;
        Token::Number(token)
    }

    pub fn next(&mut self) -> Option<Token> {
        let ch = self.input.chars().nth(self.position)?;
        self.position += 1;
        self.offset += 1;

        let tok = match ch {
            '\n' => {
                self.lineno += 1;
                self.offset = 0;
                self.next()?
            }

            '!' | '=' | '<' | '>' | '+' | '-' | '*' | '/' | '%' => {
                self.next_with_eq(ch)
            }

            ' ' | '\t' | '\r' => self.next()?,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            ';' => Token::SemiColon,
            ':' => Token::Colon,
            '.' => Token::Dot,
            'A'..='Z' | 'a'..='z' | '_' => self.read_identifier(),
            '0'..='9' => self.read_number(),
            '"' => self.read_str(),
            _ => Token::Illegal(self.position, self.offset, self.lineno)
        };

        Some(tok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let code = "import std;\
            fn foo(a: i8, b: i8) i32 {\
                return 42;\
            }\
            fn main() {\
                std.io.printf(\"Hello, World!\n\");\
                if 5 % 2 == 1 { \
                    std.io.printf(\"5 is odd\n\"); \
                } \
            }";

        let mut lex = Lexer::new(code.to_string());

        let result = vec![
            Token::Import,
            Token::Ident(String::from("std")),
            Token::SemiColon,
            Token::Fn,
            Token::Ident(String::from("foo")),
            Token::LParen,
            Token::Ident(String::from("a")),
            Token::Colon,
            Token::Ident(String::from("i8")),
            Token::Comma,
            Token::Ident(String::from("b")),
            Token::Colon,
            Token::Ident(String::from("i8")),
            Token::RParen,
            Token::Ident(String::from("i32")),
            Token::LBrace,
            Token::Return,
            Token::Number(String::from("42")),
            Token::SemiColon,
            Token::RBrace,
            Token::Fn,
            Token::Ident(String::from("main")),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("std")),
            Token::Dot,
            Token::Ident(String::from("io")),
            Token::Dot,
            Token::Ident(String::from("printf")),
            Token::LParen,
            Token::Str(String::from("Hello, World!\n")),
            Token::RParen,
            Token::SemiColon,
            Token::If,
            Token::Number(String::from("5")),
            Token::Mod,
            Token::Number(String::from("2")),
            Token::EqEq,
            Token::Number(String::from("1")),
            Token::LBrace,
            Token::Ident(String::from("std")),
            Token::Dot,
            Token::Ident(String::from("io")),
            Token::Dot,
            Token::Ident(String::from("printf")),
            Token::LParen,
            Token::Str(String::from("5 is odd\n")),
            Token::RParen,
            Token::SemiColon,
            Token::RBrace,
            Token::RBrace,
        ];

        for res in result.iter() {
            assert_eq!(*res, lex.next().unwrap())
        }
    }
}
