pub mod lexer;
pub mod parser;

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(char),
    Eof,

    Int(i32),
    String(String),
    Identifier(String),

    Plus,
    Minus,

    Dot,
    Arrow,
    Comma,
    Lparen,
    Rparen,
    Semicolon,
    FunctionDeclarator,
    If,
    Else,
    Return,

    TypeInteger,
    TypeString,
    TypeBoolean,

    True,
    False,

    GratherThan,
    Equal,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Illegal(val) => write!(f, "[Illegal Token] {}", val),
            Self::Eof => write!(f, "EOF"),
            Self::Int(int) => write!(f, "{}", int),
            Self::String(str) => write!(f, "{}", str),
            Self::Identifier(ident) => write!(f, "{}", ident),
            Self::Plus => write!(f, "+"),
            Self::Dot => write!(f, "."),
            Self::Arrow => write!(f, "->"),
            Self::Lparen => write!(f, "("),
            Self::Rparen => write!(f, ")"),
            Self::Semicolon => write!(f, ";"),
            Self::Minus => write!(f, "-"),
            Self::FunctionDeclarator => write!(f, "[Function declaration] f"),
            Self::TypeInteger => write!(f, "int"),
            Self::TypeString => write!(f, "str"),
            Self::TypeBoolean => write!(f, "bool"),
            Self::Comma => write!(f, ","),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::GratherThan => write!(f, ">"),
            Self::Return => write!(f, "return"),
            Self::Equal => write!(f, "="),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}
