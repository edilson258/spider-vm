use super::Token;

const NULL_CHAR: char = '\0';

pub struct Lexer<'a> {
    input: &'a [char],
    curr_char: char,
    pos: usize,
    read_pos: usize,
    line: usize,
    colm: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [char]) -> Self {
        let mut l = Lexer {
            input,
            curr_char: NULL_CHAR,
            pos: 0,
            read_pos: 0,
            line: 1,
            colm: 1,
        };

        l.read_char();

        l
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.pos = self.read_pos;
            self.curr_char = NULL_CHAR;
            return;
        }

        self.curr_char = self.input[self.read_pos];
        self.pos = self.read_pos;
        self.read_pos += 1;

        if self.curr_char == '\n' {
            self.line += 1;
            self.colm = 1;
        } else {
            self.colm += 1;
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        if self.eof() {
            return Ok(Token::Eof);
        }

        let token = match self.curr_char {
            '+' => Some(Token::Plus),
            '(' => Some(Token::Lparen),
            ')' => Some(Token::Rparen),
            ';' => Some(Token::Semicolon),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '>' => Some(Token::GratherThan),
            '=' => Some(Token::Equal),
            '-' => {
                if self.next_char_is('>') {
                    self.read_char();
                    Some(Token::Arrow)
                } else {
                    Some(Token::Minus)
                }
            }
            '"' => {
                let token = self.read_string();
                self.read_char();
                return token;
            }
            _ => None,
        };

        if token.is_some() {
            self.read_char();
            return Ok(token.unwrap());
        }

        if self.curr_char.is_alphabetic() {
            return Ok(self.read_identifier());
        }

        if self.curr_char.is_numeric() {
            return Ok(self.read_number());
        }

        let illegal = Token::Illegal(self.curr_char);
        self.read_char();
        Ok(illegal)
    }

    fn read_identifier(&mut self) -> Token {
        let literal = self.chop_while(|x| x.is_alphanumeric() || x == '_');
        // look for keywords
        match literal.as_str() {
            "f" => Token::FunctionDeclarator,
            "int" => Token::TypeInteger,
            "str" => Token::TypeString,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "bool" => Token::TypeBoolean,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(literal),
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.read_char();
        let literal = self.chop_while(|x| x != '"');
        if self.curr_char != '"' {
            return Err(format!("Unbalanced '\"'"));
        }
        Ok(Token::String(literal))
    }

    fn read_number(&mut self) -> Token {
        let literal = self.chop_while(|x| x.is_numeric() || x == '.');
        return Token::Int(literal.parse::<i32>().unwrap());
    }

    fn skip_whitespace(&mut self) {
        self.chop_while(|x| x.is_whitespace());
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> String
    where
        P: FnMut(char) -> bool,
    {
        let start = self.pos;
        while !self.eof() && predicate(self.curr_char) {
            self.read_char();
        }
        self.chop(start, self.pos)
    }

    fn chop(&mut self, begin: usize, end: usize) -> String {
        self.input[begin..end].iter().collect::<String>()
    }

    fn eof(&mut self) -> bool {
        self.curr_char == NULL_CHAR
    }

    fn next_char_is(&mut self, x: char) -> bool {
        if self.read_pos >= self.input.len() {
            return false;
        }
        self.input[self.read_pos] == x
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::frontend::Token;

    #[test]
    fn missing_unquote_to_balance_string_literal() {
        // "Hello world!
        let input = "\"Hello world!";
        let input = input.chars().collect::<Vec<char>>();
        let mut l = Lexer::new(&input);
        assert_eq!(Err("Unbalanced '\"'".to_string()), l.next_token());
    }

    #[test]
    fn test_next_token() {
        let input = "f main -> \"Hello, world!\" .write;";
        let input = input.chars().collect::<Vec<char>>();
        let mut l = Lexer::new(&input);
        assert_eq!(Ok(Token::FunctionDeclarator), l.next_token());
        assert_eq!(Ok(Token::Identifier("main".to_string())), l.next_token());
        assert_eq!(Ok(Token::Arrow), l.next_token());
        assert_eq!(
            Ok(Token::String("Hello, world!".to_string())),
            l.next_token()
        );
        assert_eq!(Ok(Token::Dot), l.next_token());
        assert_eq!(Ok(Token::Identifier("write".to_string())), l.next_token());
        assert_eq!(Ok(Token::Semicolon), l.next_token());
        assert_eq!(Ok(Token::Eof), l.next_token());
    }
}
