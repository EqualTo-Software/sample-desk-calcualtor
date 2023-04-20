use crate::tokens::Token;

enum NumberParseState {
    State1,
    State2,
    State3,
    State4,
    State5,
    State6,
    State7,
    State8,
}

pub struct Lexer {
    input_chars: Vec<char>,
    position: usize,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if matches!(token, Token::EoI) {
            return None;
        };
        Some(token)
    }
}

impl Lexer {
    pub fn new(input_text: &str) -> Lexer {
        let input_chars: Vec<char> = input_text.chars().collect();
        Lexer {
            input_chars,
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();
        match self.read_next_char() {
            Some(ch) => match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Times,
                '/' => Token::Divide,
                '^' => Token::Power,
                '(' => Token::OpenParenthesis,
                ')' => Token::CloseParenthesis,
                '0'..='9' | '.' => {
                    self.position -= 1;
                    self.read_number()
                }
                'A'..='Z' => {
                    self.position -= 1;
                    self.read_name()
                }
                _ => Token::Illegal(format!("Unexpected character: {}", ch)),
            },
            None => Token::EoI,
        }
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    fn read_name(&mut self) -> Token {
        // A valid function name starts with an upper letter and it is followed
        // by [a-z][A-Z]_[0-9]
        let position = self.position;
        while let Some(ch) = self.read_next_char() {
            match ch {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {}
                _ => {
                    self.position -= 1;
                    break;
                }
            }
        }

        let name: String = self.input_chars[position..self.position].iter().collect();
        Token::Name(name)
    }

    fn read_next_char(&mut self) -> Option<&char> {
        let next_char = self.input_chars.get(self.position);
        if next_char.is_some() {
            self.position += 1;
        }
        next_char
    }

    fn peek_char(&self) -> Option<&char> {
        self.input_chars.get(self.position)
    }

    fn consume_whitespace(&mut self) {
        while let Some(&char) = self.input_chars.get(self.position) {
            if !char.is_whitespace() {
                break;
            }
            self.position += 1;
        }
    }

    // DFA:
    //                         digit                       digit                                  digit
    //    +-----+-----+       +------+                   +-----+             +-----+-----+       +-----+
    //    |     +     |       |      |                   |     |             |     +     |       |     |
    //    |           |       |      |                   |     v             |           v       |     v
    // +--+--+     +--+--+    |   +--+--+     +-----+    |  +--+--+      +---+-+      +--+--+    |  +--+--+
    // |  1  |     |  2  |    +---+  3  |  .  |  4  |    +--+  5  |      |  6  |      |  7  |    +--+  8  |
    // |     |     |     +------->+     +-----+     +-------+     +----->+     |      |     +------>+     |
    // ++-+--+     +--+--+ digit  +-+-+-+     +-----+ digit +-----+  E   +-+-+-+      +--+--+       +-----+
    //  | |           ^             | |                                    ^ |           ^
    //  | |     -     |             | |                                    | |     -     |
    //  | +-----+-----+             | |                                    | +-----+-----+
    //  |                           | |                                    |
    //  |         digit             | |                 E                  |
    //  +---------------------------+ +------------------------------------+
    fn read_number(&mut self) -> Token {
        let mut state = NumberParseState::State1;
        let mut str = "".to_string();
        let mut accept = true;
        while accept {
            if let Some(&c) = self.peek_char() {
                match state {
                    NumberParseState::State1 => {
                        if c.is_ascii_digit() {
                            state = NumberParseState::State3;
                        } else if c == '-' || c == '+' {
                            state = NumberParseState::State2;
                        } else {
                            return Token::Illegal(format!("Expecting digit or + or -, got {}", c));
                        }
                    }
                    NumberParseState::State2 => {
                        if c.is_ascii_digit() {
                            state = NumberParseState::State3;
                        } else {
                            return Token::Illegal(format!("Expecting digit got  {}", c));
                        }
                    }
                    NumberParseState::State3 => {
                        // Accepting state
                        if c == '.' {
                            state = NumberParseState::State4;
                        } else if c == 'E' || c == 'e' {
                            state = NumberParseState::State6;
                        } else if !c.is_ascii_digit() {
                            accept = false;
                        }
                    }
                    NumberParseState::State4 => {
                        if c.is_ascii_digit() {
                            state = NumberParseState::State5;
                        } else {
                            return Token::Illegal(format!("Expecting digit got  {}", c));
                        }
                    }
                    NumberParseState::State5 => {
                        // Accepting state
                        if c == 'e' || c == 'E' {
                            state = NumberParseState::State6;
                        } else if !c.is_ascii_digit() {
                            accept = false;
                        }
                    }
                    NumberParseState::State6 => {
                        if c == '+' || c == '-' {
                            state = NumberParseState::State7;
                        } else if c.is_ascii_digit() {
                            state = NumberParseState::State8;
                        } else {
                            return Token::Illegal(format!("Expecting '+'or '-' got  {}", c));
                        }
                    }
                    NumberParseState::State7 => {
                        if c.is_ascii_digit() {
                            state = NumberParseState::State8;
                        } else {
                            return Token::Illegal(format!("Expecting digit got  {}", c));
                        }
                    }
                    NumberParseState::State8 => {
                        // Accepting state
                        if !c.is_ascii_digit() {
                            accept = false;
                        }
                    }
                }
                if accept {
                    str.push(c);
                    self.position += 1;
                }
            } else {
                break;
            }
        }
        Token::Number(str.parse::<f64>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::Token::*;
    use crate::{lexer::Lexer, tokens::Token};

    #[test]
    fn sums() {
        let lexer = Lexer::new("2 + 3");
        let tokens: Vec<Token> = lexer.into_iter().collect();
        assert_eq!(tokens, vec![Number(2.0), Plus, Number(3.0)]);
    }

    #[test]
    fn multiply() {
        let lexer = Lexer::new("2.3e2 * 2.3e+2");
        let tokens: Vec<Token> = lexer.into_iter().collect();
        assert_eq!(tokens, vec![Number(2.3e2), Times, Number(230.0)]);
    }

    #[test]
    fn divide() {
        let lexer = Lexer::new("500e-2 / 3");
        let tokens: Vec<Token> = lexer.into_iter().collect();
        assert_eq!(tokens, vec![Number(5.0), Divide, Number(3.0)]);
    }

    #[test]
    fn power() {
        let lexer = Lexer::new("500e-2 / 3");
        let tokens: Vec<Token> = lexer.into_iter().collect();
        assert_eq!(tokens, vec![Number(5.0), Divide, Number(3.0)]);
    }

    #[test]
    fn blog_example() {
        let lexer = Lexer::new("2.3+4*(Sin(3+7)+5)^2");
        let tokens: Vec<Token> = lexer.into_iter().collect();
        assert_eq!(
            tokens,
            vec![
                Number(2.3),
                Plus,
                Number(4.0),
                Times,
                OpenParenthesis,
                Name(String::from("Sin")),
                OpenParenthesis,
                Number(3.0),
                Plus,
                Number(7.0),
                CloseParenthesis,
                Plus,
                Number(5.0),
                CloseParenthesis,
                Power,
                Number(2.0)
            ]
        );
    }
}
