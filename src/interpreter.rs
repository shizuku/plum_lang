use crate::ast::*;
use crate::token::Token;
use std::collections::HashMap;

pub struct Interpreter {
  stack: Vec<HashMap<String, f64>>,
}

impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {
      stack: vec![HashMap::new()],
    }
  }
  fn get(&mut self, key: &String) -> Option<&f64> {
    let last_idx = self.stack.len() - 1;
    self.stack[last_idx].get(key)
  }
  fn set(&mut self, key: String, val: f64) -> Option<f64> {
    let last_idx = self.stack.len() - 1;
    self.stack[last_idx].insert(key, val)
  }
  // fn push(&mut self) {
  //   self.stack.push(HashMap::new());
  // }
  // fn pop(&mut self) {
  //   self.stack.pop();
  // }
  fn call(&mut self, e: &CallExpr) -> f64 {
    let mut args: Vec<f64> = vec![];
    for i in &e.args {
      args.push(self.visit_expr(&*i));
    }
    match &*e.fun {
      Expr::Ident(id) => match id.name.as_str() {
        "print" => {
          for i in args {
            print!("{} ", i);
          }
          f64::NAN
        }
        "println" => {
          for i in args {
            print!("{} ", i);
          }
          print!("\n");
          f64::NAN
        }
        _ => f64::NAN,
      },
      _ => f64::NAN,
    }
  }
}

impl Visitor<f64> for Interpreter {
  fn visit_file(&mut self, file: &File) -> f64 {
    let mut ret: f64 = 0.0;
    for i in &file.stmts {
      ret = self.visit_stmt(i);
    }
    ret
  }

  fn visit_stmt(&mut self, stmt: &Stmt) -> f64 {
    match stmt {
      Stmt::Bad(x) => self.visit_bad_stmt(x),
      Stmt::Decl(x) => self.visit_decl_stmt(x),
      Stmt::Expr(x) => self.visit_expr_stmt(x),
      Stmt::Assign(x) => self.visit_assign_stmt(x),
    }
  }
  fn visit_bad_stmt(&mut self, _: &BadStmt) -> f64 {
    f64::NAN
  }
  fn visit_decl_stmt(&mut self, decl_stmt: &DeclStmt) -> f64 {
    self.visit_decl(&*decl_stmt.decl)
  }
  fn visit_expr_stmt(&mut self, x: &ExprStmt) -> f64 {
    self.visit_expr(&*x.x)
  }
  fn visit_assign_stmt(&mut self, x: &AssignStmt) -> f64 {
    match &*x.ptr {
      Expr::Ident(id) => {
        let key = id.name.clone();
        let val = self.visit_expr(&*x.val);
        self.set(key, val);
        val
      }
      _ => f64::NAN,
    }
  }

  fn visit_decl(&mut self, decl: &Decl) -> f64 {
    match decl {
      Decl::Var(var_decl) => self.visit_var_decl(var_decl),
      _ => panic!(),
    }
  }
  fn visit_bad_decl(&mut self, _: &BadDecl) -> f64 {
    f64::NAN
  }
  fn visit_var_decl(&mut self, decl: &VarDecl) -> f64 {
    let key = decl.name.name.clone();
    let val = if let Option::Some(expr) = &decl.value {
      self.visit_expr(&**expr)
    } else {
      0.0
    };
    self.set(key, val);
    val
  }
  fn visit_fun_decl(&mut self, _: &FunDecl) -> f64 {
    f64::NAN
  }

  fn visit_expr(&mut self, x: &Expr) -> f64 {
    match x {
      Expr::Bad(_) => f64::NAN,
      Expr::Ident(x) => self.visit_ident(x),
      Expr::BasicLit(x) => self.visit_basic_lit(x),
      Expr::Call(x) => self.visit_call_expr(x),
      Expr::Unary(x) => self.visit_unary_expr(x),
      Expr::Binary(x) => self.visit_binary_expr(x),
      Expr::Paren(x) => self.visit_paren_expr(x),
    }
  }
  fn visit_bad_expr(&mut self, _: &BadExpr) -> f64 {
    f64::NAN
  }
  fn visit_ident(&mut self, x: &Ident) -> f64 {
    if let Option::Some(v) = self.get(&x.name) {
      *v
    } else {
      f64::NAN
    }
  }
  fn visit_basic_lit(&mut self, x: &BasicLit) -> f64 {
    match x.tok.clone() {
      Token::Integer(lit) => lit.parse::<i64>().unwrap() as f64,
      Token::Float(lit) => lit.parse::<f64>().unwrap(),
      _ => f64::NAN,
    }
  }
  fn visit_call_expr(&mut self, x: &CallExpr) -> f64 {
    self.call(x)
  }
  fn visit_unary_expr(&mut self, x: &UnaryExpr) -> f64 {
    let v = self.visit_expr(&*x.x);
    match x.op {
      Token::Add => v,
      Token::Sub => -v,
      _ => f64::NAN,
    }
  }
  fn visit_binary_expr(&mut self, x: &BinaryExpr) -> f64 {
    let xv = self.visit_expr(&*x.x);
    let yv = self.visit_expr(&*x.y);
    match x.op {
      Token::Add => xv + yv,
      Token::Sub => xv - yv,
      Token::Mul => xv * yv,
      Token::Div => xv / yv,
      Token::Rem => xv % yv,
      _ => f64::NAN,
    }
  }
  fn visit_paren_expr(&mut self, x: &ParenExpr) -> f64 {
    self.visit_expr(&*x.x)
  }
}
