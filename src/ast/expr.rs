use crate::ast::*;

#[derive(Debug)]
pub enum Expr {
  Bad(BadExpr),
  Ident(Ident),
  BasicLit(BasicLit),
  Call(CallExpr),
  Unary(UnaryExpr),
  Binary(BinaryExpr),
  Paren(ParenExpr),
}
impl Node for Expr {}
impl Printable for Expr {
  fn print(&self, l: i32) {
    match self {
      Expr::Bad(x) => x.print(l),
      Expr::Ident(x) => x.print(l),
      Expr::BasicLit(x) => x.print(l),
      Expr::Call(x) => x.print(l),
      Expr::Unary(x) => x.print(l),
      Expr::Binary(x) => x.print(l),
      Expr::Paren(x) => x.print(l),
    };
  }
}

#[derive(Debug)]
pub struct BadExpr {}
impl Node for BadExpr {}
impl Printable for BadExpr {
  fn print(&self, l: i32) {
    indent(l);
    print!("BadExpr \n");
  }
}

#[derive(Debug)]
pub struct Ident {
  pub pos: usize,
  pub name: String,
}
impl Node for Ident {}
impl Printable for Ident {
  fn print(&self, _level: i32) {
    indent(_level);
    print!("Ident {}\n", self.name);
  }
}

#[derive(Debug)]
pub struct BasicLit {
  pub pos: usize,
  pub tok: Token,
}
impl Node for BasicLit {}
impl Printable for BasicLit {
  fn print(&self, _level: i32) {
    indent(_level);
    print!("BasicLit {:?}\n", self.tok);
  }
}

#[derive(Debug)]
pub struct CallExpr {
  pub fun: Box<Expr>,
  pub lp_pos: usize,
  pub args: Vec<Box<Expr>>,
  pub rp_pos: usize,
}
impl Node for CallExpr {}
impl Printable for CallExpr {
  fn print(&self, l: i32) {
    indent(l);
    print!("CallExpr<{}, {}> {{\n", self.beg(), self.end());
    self.fun.print(l + 1);
    for i in &self.args {
      i.print(l + 2)
    }
    indent(l);
    print!("}}\n");
  }
}

/// unary expression
#[derive(Debug)]
pub struct UnaryExpr {
  pub op: Token,
  pub x: Box<Expr>,
}
impl Node for UnaryExpr {}
impl Printable for UnaryExpr {
  fn print(&self, l: i32) {
    indent(l);
    print!("UnaryExpr<{}, {}> {{\n", self.beg(), self.end());
    indent(l + 1);
    print!("{:?}\n", self.op);
    self.x.print(l + 1);
    indent(l);
    print!("}}\n");
  }
}

/// binary expression
#[derive(Debug)]
pub struct BinaryExpr {
  pub x: Box<Expr>,
  pub op: Token,
  pub y: Box<Expr>,
}
impl Node for BinaryExpr {}
impl Printable for BinaryExpr {
  fn print(&self, l: i32) {
    indent(l);
    print!("BinaryExpr<{}, {}> {{\n", self.beg(), self.end());
    self.x.print(l + 1);
    indent(l + 1);
    print!("{:?}\n", self.op);
    self.y.print(l + 1);
    indent(l);
    print!("}}\n");
  }
}

/// parenized expression
#[derive(Debug)]
pub struct ParenExpr {
  pub l_pos: usize,
  pub x: Box<Expr>,
  pub r_pos: usize,
}
impl Node for ParenExpr {}
impl Printable for ParenExpr {
  fn print(&self, l: i32) {
    indent(l);
    print!("PareExpr<{}, {}> {{\n", self.beg(), self.end());
    self.x.print(l + 1);
    indent(l);
    print!("}}\n");
  }
}
