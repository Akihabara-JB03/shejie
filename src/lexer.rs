enum Token {
  SET,
  TO,
  PRINT,
  PERIOD,
  IDENT(String),
  INT(i64),
  FLOAT(f64),
  STRLITERAL(String),
  SPACE,
  COMMENT(String),
  NEWLINE,
  PLUS,
  MINUS,
  DIV,
  TIMES,
  lLapen,
  rLapen,
}
pub struct Lexer {
  input: Vec<char>, 
  position: usize,
  ch: char,
  strmode: bool,
}
impl Lexer {
  pub fn new(inputstr: &str) -> Self {
    let inpvec: Vec<char> = inputstr.chars().collect();
    let first_ch = if !inpvec.is_empty() { inpvec[0] } else { '\0' };

    Lexer {
      input: inpvec,
      position: 0,
      ch: first_ch,
      strmode: false,
    }
  }
  pub fn next_token(&mut self) -> Token {
    match self.ch {
      ' ' => {
        if self.strmode {
          
        }
      }
    }
  }
}
