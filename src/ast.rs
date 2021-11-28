use crate::token::Token;
use std::fmt::Debug;

pub trait Node: Debug {
  fn beg(&self) -> usize {
    0
  }
  fn end(&self) -> usize {
    0
  }
}

pub trait Expr: Node {}
pub trait Type: Node {}
pub trait Stmt: Node {}
pub trait Decl: Node {}

#[derive(Debug)]
pub struct BadExpr {}
impl Node for BadExpr {}
impl Expr for BadExpr {}

#[derive(Debug)]
pub struct Ident {
  pub pos: usize,
  pub name: String,
}
impl Node for Ident {}
impl Expr for Ident {}

#[derive(Debug)]
pub struct BasicLit {
  pub pos: usize,
  pub tok: Token,
}
impl Node for BasicLit {}
impl Expr for BasicLit {}

#[derive(Debug)]
pub struct CallExpr {
  pub fun: Box<dyn Expr>,
  pub lp_pos: usize,
  pub args: Vec<Box<dyn Expr>>,
  pub rp_pos: usize,
}
impl Node for CallExpr {}
impl Expr for CallExpr {}

/// unary expression
#[derive(Debug)]
pub struct UnaryExpr {
  pub op: Token,
  pub x: Box<dyn Expr>,
}
impl Node for UnaryExpr {}
impl Expr for UnaryExpr {}

/// binary expression
#[derive(Debug)]
pub struct BinaryExpr {
  pub x: Box<dyn Expr>,
  pub op: Token,
  pub y: Box<dyn Expr>,
}
impl Node for BinaryExpr {}
impl Expr for BinaryExpr {}

/// parenized expression
#[derive(Debug)]
pub struct ParenExpr {
  pub l_pos: usize,
  pub x: Box<dyn Expr>,
  pub r_pos: usize,
}
impl Node for ParenExpr {}
impl Expr for ParenExpr {}

/// bad statement
#[derive(Debug)]
pub struct BadStmt {}
impl Node for BadStmt {}
impl Stmt for BadStmt {}

/// declaration statement
#[derive(Debug)]
pub struct DeclStmt {
  pub decl: Box<dyn Decl>,
}
impl Node for DeclStmt {}
impl Stmt for DeclStmt {}

/// expression statement
#[derive(Debug)]
pub struct ExprStmt {
  pub x: Box<dyn Expr>,
}
impl Node for ExprStmt {}
impl Stmt for ExprStmt {}

/// assignment statement
#[derive(Debug)]
pub struct AssignStmt {
  pub ptr: Box<Ident>,
  pub assign_pos: usize,
  pub val: Box<dyn Expr>,
}
impl Node for AssignStmt {}
impl Stmt for AssignStmt {}

#[derive(Debug)]
pub struct BadDecl {}
impl Node for BadDecl {}
impl Decl for BadDecl {}

/// variable declaration
#[derive(Debug)]
pub struct VarDecl {
  pub var_pos: usize,
  pub name: Box<Ident>,
  pub assign_pos: usize,
  pub value: Option<Box<dyn Expr>>,
}
impl Node for VarDecl {}
impl Decl for VarDecl {}

#[derive(Debug)]
pub struct ValDecl {
  pub val_pos: usize,
  pub name: Box<Ident>,
  pub assign_pos: usize,
  pub value: Option<Box<dyn Expr>>,
}
impl Node for ValDecl {}
impl Decl for ValDecl {}

#[derive(Debug)]
pub struct FunDecl {}
impl Node for FunDecl {}
impl Decl for FunDecl {}
