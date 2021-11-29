use crate::ast::*;

pub trait Visitor<T> {
  fn visit_file(&mut self, file: &File) -> T;

  fn visit_stmt(&mut self, x: &Stmt) -> T;
  fn visit_bad_stmt(&mut self, x: &BadStmt) -> T;
  fn visit_decl_stmt(&mut self, x: &DeclStmt) -> T;
  fn visit_expr_stmt(&mut self, x: &ExprStmt) -> T;
  fn visit_assign_stmt(&mut self, x: &AssignStmt) -> T;

  fn visit_decl(&mut self, x: &Decl) -> T;
  fn visit_bad_decl(&mut self, x: &BadDecl) -> T;
  fn visit_var_decl(&mut self, x: &VarDecl) -> T;
  fn visit_fun_decl(&mut self, x: &FunDecl) -> T;

  fn visit_expr(&mut self, x: &Expr) -> T;
  fn visit_bad_expr(&mut self, x: &BadExpr) -> T;
  fn visit_ident(&mut self, x: &Ident) -> T;
  fn visit_basic_lit(&mut self, x: &BasicLit) -> T;
  fn visit_call_expr(&mut self, x: &CallExpr) -> T;
  fn visit_unary_expr(&mut self, x: &UnaryExpr) -> T;
  fn visit_binary_expr(&mut self, x: &BinaryExpr) -> T;
  fn visit_paren_expr(&mut self, x: &ParenExpr) -> T;
}
