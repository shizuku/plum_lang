use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Token {
  Illegal,
  Comment(String),
  Eof,

  Ident(String),
  Integer(String),
  Float(String),
  String(String),

  Add, // +
  Sub, // -
  Mul, // *
  Div, // /
  Rem, // %

  Assign, // =

  Lor,  // ||
  Land, // &&
  Lss,  // <
  Leq,  // <=
  Gtr,  // >
  Geq,  // >=

  Eql, // ==
  Neq, // !=

  Lparen, // (
  Rparen, // )
  Lbrack, // [
  Rbrack, // ]
  Lbrace, // {
  Rbrace, // }

  Comma,     // ,
  Semicolon, // ;
  Colon,     // :
  Exel,      // !

  Fun,
  Var,
  Val,
  Import,
  Return,
}

pub const LOWEST_PREC: i32 = 0;
pub const UNARY_PREC: i32 = 6;
pub const HIGHEST_PREC: i32 = 7;

impl Token {
  pub fn prec(&self) -> i32 {
    match self {
      Token::Lor => 1,
      Token::Land => 2,
      Token::Eql | Token::Neq | Token::Gtr | Token::Geq | Token::Lss | Token::Leq => 3,
      Token::Add | Token::Sub => 4,
      Token::Mul | Token::Div | Token::Rem => 5,
      _ => LOWEST_PREC,
    }
  }
}
