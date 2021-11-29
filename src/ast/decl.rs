use crate::ast::*;

#[derive(Debug)]
pub enum Decl {
  Bad(BadDecl),
  Var(VarDecl),
  Fun(FunDecl),
}
impl Node for Decl {}
impl Printable for Decl {
  fn print(&self, l: i32) {
    match self {
      Decl::Bad(x) => x.print(l),
      Decl::Var(x) => x.print(l),
      Decl::Fun(x) => x.print(l),
    }
  }
}

#[derive(Debug)]
pub struct BadDecl {}
impl Node for BadDecl {}
impl Printable for BadDecl {
  fn print(&self, l: i32) {
    indent(l);
    print!("BadDecl \n");
  }
}

/// variable declaration
#[derive(Debug)]
pub struct VarDecl {
  pub var_pos: usize,
  pub name: Box<Ident>,
  pub assign_pos: usize,
  pub value: Option<Box<Expr>>,
}
impl Node for VarDecl {}
impl Printable for VarDecl {
  fn print(&self, l: i32) {
    indent(l);
    print!("VarDecl<{}, {}> {{\n", self.beg(), self.end());
    self.name.print(l + 1);
    if let Option::Some(v) = &self.value {
      v.print(l + 1);
    }
    indent(l);
    print!("}}\n");
  }
}

#[derive(Debug)]
pub struct FunDecl {}
impl Node for FunDecl {}
impl Printable for FunDecl {
  fn print(&self, l: i32) {
    indent(l);
    print!("FunDecl<{}, {}> {{\n", self.beg(), self.end());

    indent(l);
    print!("}}\n");
  }
}
