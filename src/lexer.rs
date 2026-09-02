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
