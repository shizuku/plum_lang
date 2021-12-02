pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;

use std::fs;

use ast::*;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

use clap::Parser as AParser;

#[derive(AParser)]
#[clap(version = "0.0.1", author = "shizuku")]
struct Command {
  input: String,

  #[clap(long)]
  ast: bool,

  #[clap(long)]
  tok: bool,
}

fn main() {
  let c: Command = Command::parse();
  if c.ast {
    return print_ast(c);
  }
  if c.tok {
    return print_tokens(c);
  }
  interpret(c);
}

fn interpret(c: Command) {
  let lexer = if let Ok(s) = fs::read_to_string(c.input) {
    Lexer::new(s)
  } else {
    panic!("file error");
  };
  let mut parser = Parser::new(lexer);
  let ast = parser.parse_file();
  if parser.errors.len() != 0 {
    for i in parser.errors {
      println!("{} {:?}", i.0, i.1);
    }
    panic!("parser errors")
  }
  let mut interpreter = Interpreter::new();
  interpreter.visit_file(&*ast);
}

fn print_ast(c: Command) {
  let lexer = if let Ok(s) = fs::read_to_string(c.input) {
    Lexer::new(s)
  } else {
    panic!("file error");
  };
  let mut parser = Parser::new(lexer);
  let ast = parser.parse_file();
  if parser.errors.len() != 0 {
    for i in parser.errors {
      println!("{} {:?}", i.0, i.1);
    }
    panic!("parser errors")
  }
  ast.print(0);
}

fn print_tokens(c: Command) {
  let mut lexer = if let Ok(s) = fs::read_to_string(c.input) {
    Lexer::new(s)
  } else {
    panic!("file error");
  };
  let tokens = lexer.lex_all();
  for i in tokens {
    println!("{:?}", i);
  }
}
