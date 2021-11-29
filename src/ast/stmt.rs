use crate::ast::*;

#[derive(Debug)]
pub enum Stmt {
  Bad(BadStmt),
  Decl(DeclStmt),
  Expr(ExprStmt),
  Assign(AssignStmt),
}
impl Node for Stmt {}
impl Printable for Stmt {
  fn print(&self, l: i32) {
    match self {
      Stmt::Bad(x) => x.print(l),
      Stmt::Decl(x) => x.print(l),
      Stmt::Expr(x) => x.print(l),
      Stmt::Assign(x) => x.print(l),
    }
  }
}

/// bad statement
#[derive(Debug)]
pub struct BadStmt {}
impl Node for BadStmt {}
impl Printable for BadStmt {
  fn print(&self, l: i32) {
    indent(l);
    print!("BadStmt \n");
  }
}

/// declaration statement
#[derive(Debug)]
pub struct DeclStmt {
  pub decl: Box<Decl>,
}
impl Node for DeclStmt {}
impl Printable for DeclStmt {
  fn print(&self, l: i32) {
    indent(l);
    print!("DeclStmt<{}, {}> {{\n", self.beg(), self.end());
    self.decl.print(l + 1);
    indent(l);
    print!("}}\n");
  }
}

/// expression statement
#[derive(Debug)]
pub struct ExprStmt {
  pub x: Box<Expr>,
}
impl Node for ExprStmt {}
impl Printable for ExprStmt {
  fn print(&self, l: i32) {
    indent(l);
    print!("ExprStmt<{}, {}> {{\n", self.beg(), self.end());
    self.x.print(l + 1);
    indent(l);
    print!("}}\n");
  }
}

/// assignment statement
#[derive(Debug)]
pub struct AssignStmt {
  pub ptr: Box<Expr>,
  pub pos: usize,
  pub tok: Token,
  pub val: Box<Expr>,
}
impl Node for AssignStmt {}
impl Printable for AssignStmt {
  fn print(&self, l: i32) {
    indent(l);
    print!("AssignStmt<{}, {}> {{\n", self.beg(), self.end());
    self.ptr.print(l + 1);
    self.val.print(l + 1);
    indent(l);
    print!("}}\n");
  }
}
