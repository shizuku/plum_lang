pub mod decl;
pub mod expr;
pub mod stmt;
pub mod visit;

use std::fmt::Debug;

use crate::token::Token;

pub use decl::*;
pub use expr::*;
pub use stmt::*;
pub use visit::*;

pub trait Printable {
  fn print(&self, l: i32);
}

pub trait Node: Debug + Printable {
  fn beg(&self) -> usize {
    0
  }
  fn end(&self) -> usize {
    0
  }
}

fn indent(level: i32) {
  for _ in 0..level {
    print!(".   ")
  }
}

#[derive(Debug)]
pub struct File {
  pub stmts: Vec<Box<Stmt>>,
}
impl Node for File {
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
impl Printable for File {
  fn print(&self, l: i32) {
    indent(l);
    print!("File<{}, {}> {{\n", self.beg(), self.end());
    for i in &self.stmts {
      i.print(l + 1);
    }
    indent(l);
    print!("}}\n");
  }
}

pub trait Type: Node {}
