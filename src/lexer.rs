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
    let first_ch = if !inpvec.is_empty() { inpvec[0] } else { '\0' };

    Lexer {
      input: inpvec,
      position: 0,
      ch: first_ch,
    }
  }
  pub fn next_token(&mut self) -> Token {

  }
}
