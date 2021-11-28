use crate::token::Token;
use std::fmt::Debug;

pub trait Node: Debug {
  fn beg(&self) -> usize {
    0
  }
  fn end(&self) -> usize {
    0
  }
  fn print(&self, _: i32) {}
}

fn print_indent(level: i32) {
  for _ in 0..level {
    print!(".   ")
  }
}

#[derive(Debug)]
pub struct File {
  pub stmts: Vec<Box<dyn Stmt>>,
}
impl Node for File {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("File<{}, {}> {{\n", self.beg(), self.end());
    for i in &self.stmts {
      i.print(l + 1);
    }
    print_indent(l);
    print!("}}\n");
  }
  fn beg(&self) -> usize {
    if self.stmts.len() > 0 {
      self.stmts[0].beg()
    } else {
      0
    }
  }
  fn end(&self) -> usize {
    if self.stmts.len() > 0 {
      self.stmts[self.stmts.len() - 1].beg()
    } else {
      0
    }
  }
}

pub trait Expr: Node {}
pub trait Type: Node {}
pub trait Stmt: Node {}
pub trait Decl: Node {}

#[derive(Debug)]
pub struct BadExpr {}
impl Node for BadExpr {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("BadExpr \n");
  }
}
impl Expr for BadExpr {}

#[derive(Debug)]
pub struct Ident {
  pub pos: usize,
  pub name: String,
}
impl Node for Ident {
  fn print(&self, _level: i32) {
    print_indent(_level);
    print!("Ident {}\n", self.name);
  }
}
impl Expr for Ident {}

#[derive(Debug)]
pub struct BasicLit {
  pub pos: usize,
  pub tok: Token,
}
impl Node for BasicLit {
  fn print(&self, _level: i32) {
    print_indent(_level);
    print!("BasicLit {:?}\n", self.tok);
  }
}
impl Expr for BasicLit {}

#[derive(Debug)]
pub struct CallExpr {
  pub fun: Box<dyn Expr>,
  pub lp_pos: usize,
  pub args: Vec<Box<dyn Expr>>,
  pub rp_pos: usize,
}
impl Node for CallExpr {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("CallExpr<{}, {}> {{\n", self.beg(), self.end());
    self.fun.print(l + 1);
    for i in &self.args {
      i.print(l + 2)
    }
    print_indent(l);
    print!("}}\n");
  }
}
impl Expr for CallExpr {}

/// unary expression
#[derive(Debug)]
pub struct UnaryExpr {
  pub op: Token,
  pub x: Box<dyn Expr>,
}
impl Node for UnaryExpr {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("UnaryExpr<{}, {}> {{\n", self.beg(), self.end());
    print_indent(l + 1);
    print!("{:?}\n", self.op);
    self.x.print(l + 1);
    print_indent(l);
    print!("}}\n");
  }
}
impl Expr for UnaryExpr {}

/// binary expression
#[derive(Debug)]
pub struct BinaryExpr {
  pub x: Box<dyn Expr>,
  pub op: Token,
  pub y: Box<dyn Expr>,
}
impl Node for BinaryExpr {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("BinaryExpr<{}, {}> {{\n", self.beg(), self.end());
    self.x.print(l + 1);
    print_indent(l + 1);
    print!("{:?}\n", self.op);
    self.y.print(l + 1);
    print_indent(l);
    print!("}}\n");
  }
}
impl Expr for BinaryExpr {}

/// parenized expression
#[derive(Debug)]
pub struct ParenExpr {
  pub l_pos: usize,
  pub x: Box<dyn Expr>,
  pub r_pos: usize,
}
impl Node for ParenExpr {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("PareExpr<{}, {}> {{\n", self.beg(), self.end());
    self.x.print(l + 1);
    print_indent(l);
    print!("}}\n");
  }
}
impl Expr for ParenExpr {}

/// bad statement
#[derive(Debug)]
pub struct BadStmt {}
impl Node for BadStmt {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("BadStmt \n");
  }
}
impl Stmt for BadStmt {}

/// declaration statement
#[derive(Debug)]
pub struct DeclStmt {
  pub decl: Box<dyn Decl>,
}
impl Node for DeclStmt {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("DeclStmt<{}, {}> {{\n", self.beg(), self.end());
    self.decl.print(l + 1);
    print_indent(l);
    print!("}}\n");
  }
}
impl Stmt for DeclStmt {}

/// expression statement
#[derive(Debug)]
pub struct ExprStmt {
  pub x: Box<dyn Expr>,
}
impl Node for ExprStmt {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("ExprStmt<{}, {}> {{\n", self.beg(), self.end());
    self.x.print(l + 1);
    print_indent(l);
    print!("}}\n");
  }
}
impl Stmt for ExprStmt {}

/// assignment statement
#[derive(Debug)]
pub struct AssignStmt {
  pub ptr: Box<Ident>,
  pub assign_pos: usize,
  pub val: Box<dyn Expr>,
}
impl Node for AssignStmt {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("AssignStmt<{}, {}> {{\n", self.beg(), self.end());

    print_indent(l);
    print!("}}\n");
  }
}
impl Stmt for AssignStmt {}

#[derive(Debug)]
pub struct BadDecl {}
impl Node for BadDecl {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("BadDecl \n");
  }
}
impl Decl for BadDecl {}

/// variable declaration
#[derive(Debug)]
pub struct VarDecl {
  pub var_pos: usize,
  pub name: Box<Ident>,
  pub assign_pos: usize,
  pub value: Option<Box<dyn Expr>>,
}
impl Node for VarDecl {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("VarDecl<{}, {}> {{\n", self.beg(), self.end());
    self.name.print(l + 1);
    if let Option::Some(v) = &self.value {
      v.print(l + 1);
    }
    print_indent(l);
    print!("}}\n");
  }
}
impl Decl for VarDecl {}

#[derive(Debug)]
pub struct ValDecl {
  pub val_pos: usize,
  pub name: Box<Ident>,
  pub assign_pos: usize,
  pub value: Option<Box<dyn Expr>>,
}
impl Node for ValDecl {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("ValDecl<{}, {}> {{\n", self.beg(), self.end());
    self.name.print(l + 1);
    if let Option::Some(v) = &self.value {
      v.print(l + 1);
    }
    print_indent(l);
    print!("}}\n");
  }
}
impl Decl for ValDecl {}

#[derive(Debug)]
pub struct FunDecl {}
impl Node for FunDecl {
  fn print(&self, l: i32) {
    print_indent(l);
    print!("FunDecl<{}, {}> {{\n", self.beg(), self.end());

    print_indent(l);
    print!("}}\n");
  }
}
impl Decl for FunDecl {}
