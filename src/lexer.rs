use crate::token::Token;

pub struct Lexer {
  chars: Vec<char>,
  ch: char,
  offset: usize,
  insert_semi: bool,
}

impl Lexer {
  pub fn new(src: String) -> Lexer {
    let mut ret = Lexer {
      chars: src.chars().collect(),
      offset: 0,
      ch: ' ',
      insert_semi: false,
    };
    ret.next();
    ret
  }

  pub fn lex_all(&mut self) -> Vec<(Token, usize)> {
    let mut tokens: Vec<(Token, usize)> = vec![];
    loop {
      let (tok, pos) = self.lex();
      if let Token::Eof = tok {
        break;
      }
      tokens.push((tok, pos));
    }
    tokens
  }

  pub fn lex(&mut self) -> (Token, usize) {
    self.skip_white_space();
    let pos = self.offset;
    let ch = self.ch;
    let mut ist = false;
    let tok = if is_letter(ch) || ch == '_' {
      let (t, i) = lookup(self.lex_ident());
      if i {
        ist = i;
      }
      (t, pos)
    } else if is_number(ch) {
      ist = true;
      self.lex_number()
    } else {
      self.next();
      match ch {
        '\0' => {
          if self.insert_semi {
            self.insert_semi = false;
            return (Token::Semicolon, pos);
          }
          (Token::Eof, pos)
        }
        '\n' => {
          self.insert_semi = false;
          return (Token::Semicolon, pos);
        }
        '"' => self.lex_string(),
        '(' => (Token::Lparen, pos),
        ')' => {
          ist = true;
          (Token::Rparen, pos)
        }
        '[' => (Token::Lbrack, pos),
        ']' => {
          ist = true;
          (Token::Rbrack, pos)
        }
        '{' => (Token::Lbrace, pos),
        '}' => {
          ist = true;
          (Token::Rbrace, pos)
        }
        ',' => (Token::Comma, pos),
        ';' => (Token::Semicolon, pos),
        ':' => (Token::Colon, pos),
        '!' => (Token::Exel, pos),
        '=' => (Token::Assign, pos),
        '+' => (Token::Add, pos),
        '-' => (Token::Sub, pos),
        '*' => (Token::Mul, pos),
        '/' => (Token::Div, pos),
        '%' => (Token::Rem, pos),
        _ => {
          ist = self.insert_semi;
          (Token::Illegal, pos)
        }
      }
    };
    self.insert_semi = ist;
    tok
  }

  fn lex_ident(&mut self) -> Token {
    let mut lit = String::from("");
    while is_letter(self.ch) || is_dec(self.ch) || self.ch == '_' {
      lit.push(self.ch);
      self.next();
    }
    Token::Ident(lit)
  }

  fn lex_number(&mut self) -> (Token, usize) {
    let pos = self.offset;
    let mut lit = String::from("");
    while is_number(self.ch) || self.ch == '.' || self.ch == '_' {
      lit.push(self.ch);
      self.next();
    }
    (Token::Integer(lit), pos)
  }

  fn lex_string(&mut self) -> (Token, usize) {
    let pos = self.offset - 1;
    let mut lit = String::from("");
    loop {
      let c = self.ch;
      if c == '\n' || c == '\0' {
        break;
      }
      self.next();
      if c == '"' {
        break;
      }
      lit.push(c);
    }
    (Token::String(lit), pos)
  }
}

impl Lexer {
  fn skip_white_space(&mut self) {
    while self.ch == ' '
      || self.ch == '\t'
      || (self.ch == '\n' && !self.insert_semi)
      || self.ch == '\r'
    {
      self.next();
    }
  }

  fn next(&mut self) {
    if self.offset < self.chars.len() {
      self.ch = self.chars[self.offset];
      self.offset += 1;
    } else {
      self.ch = '\0';
      self.offset = self.chars.len();
    }
  }
}

fn is_letter(ch: char) -> bool {
  (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')
}

fn is_number(ch: char) -> bool {
  is_dec(ch)
}

fn is_dec(ch: char) -> bool {
  ch >= '0' && ch <= '9'
}

// fn is_hex(ch: char) -> bool {
//   (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F')
// }

fn lookup(tok: Token) -> (Token, bool) {
  if let Token::Ident(lit) = tok {
    match lit.as_str() {
      "fun" => (Token::Fun, false),
      "var" => (Token::Var, false),
      "import" => (Token::Import, false),
      "return" => (Token::Return, true),
      _ => (Token::Ident(lit), true),
    }
  } else {
    (tok, false)
  }
}
