pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use std::env;
use std::fs;

use lexer::Lexer;
use parser::Parser;

fn main() {
  let args: Vec<String> = env::args().collect();
  let path = &args[1];

  let lexer = if let Ok(s) = fs::read_to_string(path) {
    Lexer::new(s)
  } else {
    panic!("file error");
  };
  // for tok in lexer.lex_all() {
  //   println!("{:?}", tok)
  // }
  let mut parser = Parser::new(lexer);
  let ast = parser.parse_file();
  println!("{:?}", ast);
}
