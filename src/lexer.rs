enum Token {
  SET,
  TO,
  PRINT,
  PERIOD,
  IDENT(String),
  INT(i64),
  FLOAT(f64),
  STRLITERAL(String)
}
pub struct Lexer {
  input: Vec<char>, 
  position: usize,
  ch: char,
}
impl lexer() {
  pub fn new(inputstr: &str) -> Self {
    let inpvec: Vec<char> = inputstr.chars().collect();
  }
}
