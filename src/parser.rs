use crate::ast::{
  BadDecl, BadExpr, BadStmt, BasicLit, BinaryExpr, CallExpr, Decl, DeclStmt, Expr, ExprStmt, Ident,
  ParenExpr, Stmt, UnaryExpr, ValDecl, VarDecl,
};
use crate::lexer::Lexer;
use crate::token::{Token, LOWEST_PREC};

pub struct Parser {
  tok: Token,
  pos: usize,
  lexer: Lexer,
  errors: Vec<(usize, String)>,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Parser {
    let mut p = Parser {
      tok: Token::Illegal,
      pos: 0,
      lexer,
      errors: Vec::new(),
    };
    p.next();
    p
  }

  pub fn parse_file(&mut self) -> Vec<Box<dyn Stmt>> {
    self.parse_stmts()
  }

  fn next(&mut self) {
    let t = self.lexer.lex();
    self.tok = t.0;
    self.pos = t.1;
  }

  fn tok_prec(&self) -> i32 {
    self.tok.prec()
  }

  fn error(&mut self, pos: usize, msg: String) {
    self.errors.push((pos, msg))
  }
}

/// statements
impl Parser {
  fn parse_stmts(&mut self) -> Vec<Box<dyn Stmt>> {
    let mut ret: Vec<Box<dyn Stmt>> = vec![];
    loop {
      if let Token::Eof = self.tok {
        break;
      }
      ret.push(self.parse_stmt());
    }
    ret
  }

  fn parse_stmt(&mut self) -> Box<dyn Stmt> {
    return match self.tok {
      Token::Var | Token::Val => self.parse_decl_stmt(),
      Token::Ident(_)
      | Token::Integer(_)
      | Token::Float(_)
      | Token::String(_)
      | Token::Add
      | Token::Sub => self.parse_simple_stmt(),
      _ => Box::new(BadStmt {}),
    };
  }

  fn parse_decl_stmt(&mut self) -> Box<dyn Stmt> {
    let decl: Box<dyn Decl> = self.parse_decl();
    if let Token::Semicolon = self.tok {
      self.next();
    } else {
      self.error(self.pos, String::from("expected semiclon"))
    }
    Box::new(DeclStmt { decl })
  }

  fn parse_decl(&mut self) -> Box<dyn Decl> {
    match self.tok {
      Token::Var => {
        let var_pos = self.pos;
        self.next();
        self.parse_var_decl(var_pos)
      }
      Token::Val => {
        let val_pos = self.pos;
        self.next();
        self.parse_val_decl(val_pos)
      }
      _ => Box::new(BadDecl {}),
    }
  }

  fn parse_var_decl(&mut self, var_pos: usize) -> Box<dyn Decl> {
    // 'var' has been eaten
    let name = self.parse_ident();
    if let Token::Assign = self.tok {
      let assign_pos = self.pos;
      self.next();
      let value = self.parse_expr();
      Box::new(VarDecl {
        var_pos,
        name,
        assign_pos,
        value: Option::from(value),
      })
    } else {
      Box::new(VarDecl {
        var_pos,
        name,
        assign_pos: 0,
        value: Option::None,
      })
    }
  }

  fn parse_val_decl(&mut self, val_pos: usize) -> Box<dyn Decl> {
    // 'val' has been eaten
    let name = self.parse_ident();
    if let Token::Assign = self.tok {
      let assign_pos = self.pos;
      self.next();
      let value = self.parse_expr();
      Box::new(ValDecl {
        val_pos,
        name,
        assign_pos,
        value: Option::from(value),
      })
    } else {
      Box::new(ValDecl {
        val_pos,
        name,
        assign_pos: 0,
        value: Option::None,
      })
    }
  }

  fn parse_simple_stmt(&mut self) -> Box<dyn Stmt> {
    let x = self.parse_expr();
    if let Token::Semicolon = self.tok {
      self.next();
    } else {
      self.error(self.pos, String::from("expected semiclon"))
    }
    Box::new(ExprStmt { x })
  }
}

/// expressions
impl Parser {
  /// expr ::= binaryExpr
  ///        | unaryExpr
  ///        | primaryExpr
  ///        | operand
  ///        | callExpr;
  fn parse_expr(&mut self) -> Box<dyn Expr> {
    self.parse_binary_expr(Option::None, LOWEST_PREC + 1)
  }

  /// binaryExpr ::= expr op expr;
  fn parse_binary_expr(&mut self, xx: Option<Box<dyn Expr>>, prec: i32) -> Box<dyn Expr> {
    let mut x = if let Option::Some(v) = xx {
      v
    } else {
      self.parse_unary_expr()
    };
    loop {
      let pr = self.tok_prec();
      if pr < prec {
        return x;
      }
      let op = self.tok.clone();
      self.next();
      let y = self.parse_binary_expr(Option::None, prec + 1);
      x = Box::new(BinaryExpr { x, op, y })
    }
  }

  /// unaryExpr ::= op expr;
  fn parse_unary_expr(&mut self) -> Box<dyn Expr> {
    match self.tok {
      Token::Add | Token::Sub => {
        let op = self.tok.clone();
        self.next();
        let x = self.parse_unary_expr();
        Box::new(UnaryExpr { op, x })
      }
      _ => self.parse_primary_expr(Option::None),
    }
  }

  /// primaryExpr ::= operand
  ///               | operand '(' exprList ')' # callExpr;
  fn parse_primary_expr(&mut self, xx: Option<Box<dyn Expr>>) -> Box<dyn Expr> {
    let mut x = if let Option::Some(v) = xx {
      v
    } else {
      self.parse_operand()
    };
    loop {
      match self.tok {
        Token::Lparen => x = self.parse_call_expr(Option::from(x)),
        // Token::Lbrack
        // Token::Lbrace
        _ => return x,
      }
    }
  }

  /// operand ::= Ident
  ///           | basicLit
  ///           | '(' expr ')';
  fn parse_operand(&mut self) -> Box<dyn Expr> {
    match self.tok.clone() {
      Token::Ident(lit) => {
        let pos = self.pos;
        self.next();
        Box::new(Ident {
          pos,
          name: lit.clone(),
        })
      }
      Token::Integer(_) | Token::String(_) => {
        let pos = self.pos;
        let tok = self.tok.clone();
        self.next();
        Box::new(BasicLit { pos, tok })
      }
      Token::Lparen => {
        let l_pos = self.pos;
        self.next();
        let x = self.parse_expr();
        let r_pos = if let Token::Rparen = self.tok {
          let rp = self.pos;
          self.next();
          rp
        } else {
          self.error(self.pos, String::from("expected ')'"));
          0
        };
        Box::new(ParenExpr { l_pos, x, r_pos })
      }
      _ => {
        self.next();
        Box::new(BadExpr {})
      }
    }
  }

  /// callExpr ::= operand '(' exprList ')';
  fn parse_call_expr(&mut self, xx: Option<Box<dyn Expr>>) -> Box<dyn Expr> {
    let fun = if let Option::Some(v) = xx {
      v
    } else {
      self.parse_unary_expr()
    };
    let lp_pos = if let Token::Lparen = self.tok {
      let lp = self.pos;
      self.next();
      lp
    } else {
      self.error(self.pos, String::from("expect Lparen"));
      0
    };
    let args = self.parse_expr_list(Option::None);
    let rp_pos = if let Token::Rparen = self.tok {
      let rp = self.pos;
      self.next();
      rp
    } else {
      self.error(self.pos, String::from("expect Rparen"));
      0
    };
    Box::from(CallExpr {
      fun,
      lp_pos,
      args,
      rp_pos,
    })
  }

  /// exprList ::= expr (',' expr)*;
  fn parse_expr_list(&mut self, xx: Option<Box<dyn Expr>>) -> Vec<Box<dyn Expr>> {
    let x = if let Option::Some(v) = xx {
      v
    } else {
      self.parse_unary_expr()
    };
    let mut ret: Vec<Box<dyn Expr>> = vec![x];
    while let Token::Comma = self.tok {
      self.next(); // eat comma
      ret.push(self.parse_expr())
    }
    ret
  }

  /// Ident
  fn parse_ident(&mut self) -> Box<Ident> {
    let (pos, name) = if let Token::Ident(lit) = self.tok.clone() {
      let pos = self.pos;
      self.next();
      (pos, lit)
    } else {
      self.error(self.pos, String::from("expect Identifier"));
      self.next();
      (0, String::from("_"))
    };
    Box::from(Ident { pos, name })
  }
}
