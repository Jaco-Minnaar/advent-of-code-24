use std::{io::Read, iter, str::Chars};

use advent_of_code::get_input;

#[derive(Debug, PartialEq)]
enum Token {
    Mul,
    Do,
    Dont,
    LeftParen,
    RightParen,
    Num(u32),
    Comma,
    Rubbish,
}

const EOF_CHAR: char = '\0';

pub struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            initial_len: input.len(),
            chars: input.chars(),
        }
    }

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub fn reset_len_consumed(&mut self) {
        self.initial_len = self.chars.as_str().len()
    }

    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        Some(c)
    }

    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}

impl Cursor<'_> {
    fn advance_token(&mut self) -> Token {
        let c = match self.bump() {
            Some(c) => c,
            None => EOF_CHAR,
        };

        match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ',' => Token::Comma,
            'm' => {
                if self.first() == 'u' && self.second() == 'l' {
                    self.bump().unwrap();
                    self.bump().unwrap();
                    Token::Mul
                } else {
                    Token::Rubbish
                }
            }
            'd' => {
                if self.first() == 'o' {
                    self.bump().unwrap();
                    if self.first() == 'n' && self.second() == '\'' {
                        self.bump().unwrap();
                        self.bump().unwrap();
                        if self.first() == 't' && self.second() == '(' {
                            self.bump().unwrap();
                            self.bump().unwrap();
                            if self.first() == ')' {
                                self.bump().unwrap();
                                Token::Dont
                            } else {
                                Token::Rubbish
                            }
                        } else {
                            Token::Rubbish
                        }
                    } else if self.first() == '(' && self.second() == ')' {
                        self.bump().unwrap();
                        self.bump().unwrap();
                        Token::Do
                    } else {
                        Token::Rubbish
                    }
                } else {
                    Token::Rubbish
                }
            }
            c if c.is_ascii_digit() => {
                let mut num_str = String::new();
                num_str.push(c);

                while self.first().is_ascii_digit() {
                    let c = self.bump().unwrap();
                    num_str.push(c);
                }

                let num: u32 = num_str.parse::<u32>().unwrap();
                Token::Num(num)
            }
            _ => Token::Rubbish,
        }
    }
}

pub fn main() {
    let mut input = String::new();
    get_input(3).read_to_string(&mut input).unwrap();

    let sum = eval(&input, true);

    println!("{sum}");
}

fn eval(input: &str, use_conditionals: bool) -> u32 {
    let mut tokens = tokens(input);
    let mut sum = 0;

    let mut enabled = true;
    while let Some(token) = tokens.next() {
        if use_conditionals {
            if token == Token::Do {
                enabled = true;
                continue;
            }

            if token == Token::Dont {
                enabled = false;
                continue;
            }
        }

        if token != Token::Mul {
            continue;
        }

        let Some(token) = tokens.next() else {
            break;
        };
        if token != Token::LeftParen {
            continue;
        }

        let Some(token) = tokens.next() else {
            break;
        };
        let Token::Num(num1) = token else {
            continue;
        };

        let Some(token) = tokens.next() else {
            break;
        };
        if token != Token::Comma {
            continue;
        }

        let Some(token) = tokens.next() else {
            break;
        };
        let Token::Num(num2) = token else {
            continue;
        };

        let Some(token) = tokens.next() else {
            break;
        };
        if token != Token::RightParen {
            continue;
        }

        if enabled {
            sum += num1 * num2;
        }
    }

    sum
}

fn tokens(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            cursor.reset_len_consumed();
            Some(cursor.advance_token())
        }
    })
}

#[cfg(test)]
mod test {
    use crate::{eval, tokens, Token};

    #[test]
    fn tokenize_basic_input() {
        let input = "mul(123,4)";
        let tokens: Vec<Token> = tokens(input).collect();

        let expected = [
            Token::Mul,
            Token::LeftParen,
            Token::Num(123),
            Token::Comma,
            Token::Num(4),
            Token::RightParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn tokenize_basic_input_with_mistake() {
        let input = "mul(6,9!";
        let tokens: Vec<Token> = tokens(input).collect();

        let expected = [
            Token::Mul,
            Token::LeftParen,
            Token::Num(6),
            Token::Comma,
            Token::Num(9),
            Token::Rubbish,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn tokenize_input_with_conditional() {
        let input = "don't()mul(2,3)do()mul(3,2)";
        let tokens: Vec<Token> = tokens(input).collect();

        let expected = [
            Token::Dont,
            Token::Mul,
            Token::LeftParen,
            Token::Num(2),
            Token::Comma,
            Token::Num(3),
            Token::RightParen,
            Token::Do,
            Token::Mul,
            Token::LeftParen,
            Token::Num(3),
            Token::Comma,
            Token::Num(2),
            Token::RightParen,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn eval_basic_input() {
        let input = "mul(123,4)";
        let result = eval(input, false);
        let expected = 123 * 4;

        assert_eq!(result, expected);
    }

    #[test]
    fn eval_complex_input_with_mistakes() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = eval(input, false);
        let expected = 161;

        assert_eq!(result, expected);
    }

    #[test]
    fn eval_complex_input_with_mistakes_and_conditionals() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = eval(input, true);
        let expected = 48;

        assert_eq!(result, expected);
    }
}
