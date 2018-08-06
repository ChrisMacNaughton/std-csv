use std::io::BufRead;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_parses_a_part() {
        let input = Cursor::new("part,another part");
        let mut parser = Parser::new(input);
        assert_eq!("part", parser.next_part().unwrap());
    }

    #[test]
    fn it_handles_newline_in_row() {
        let input = Cursor::new("\"quoted\nnewline\"");
        let expected = "quoted\nnewline";
        let mut parser = Parser::new(input);
        assert_eq!(expected, parser.next_part().unwrap());
    }

    #[test]
    fn it_returns_a_line() {
        let input = Cursor::new("part,another part");
        let mut parser = Parser::new(input);

        assert_eq!(vec!["part", "another part"], parser.next().unwrap());
    }

    #[test]
    fn it_handles_multiple_lines() {
        let input_raw = r#"name,dob,email
test1,01-12-1980,test1@example.com
test2,20-2-1970,test2@example.com
"#;
        let input = Cursor::new(input_raw);
        let mut parser = Parser::new(input);
        let expected = vec![
            vec!["name", "dob", "email"],
            vec!["test1", "01-12-1980", "test1@example.com"],
            vec!["test2", "20-2-1970", "test2@example.com"]
        ];
        // while let Some(row) = parser.next() {
        //     println!("Row: {:?}", row);
        // }
        assert_eq!(expected[0], parser.next().unwrap());
        assert_eq!(expected[1], parser.next().unwrap());
        assert_eq!(expected[2], parser.next().unwrap());
        assert_eq!(None, parser.next());
    }
}

const QUOTE: u8 = '"' as u8;
const COMMA: u8 = ',' as u8;
const NEWLINE: u8 = '\n' as u8;
const LINEFEED: u8 = '\r' as u8;

pub struct Parser<T: BufRead> {
    reader: T,
    eof: bool,
    quoted: bool,
}

#[derive(Debug)]
enum ParseError {
    Eol,
    Eof,
}

impl <T: BufRead>Parser<T> {
    pub fn new(reader: T) -> Parser<T> {
        Parser {
            reader,
            eof: false,
            quoted: false
        }
    }

    fn next_part(&mut self) -> Result<String, ParseError> {
        let (consumed, bytes, err): (usize, Vec<u8>, Option<ParseError>) = {
            let mut err = None;
            let mut buff = self.reader.fill_buf().map_err(|_| ParseError::Eof)?.iter().peekable();
            if buff.len() == 0 {
                err = Some(ParseError::Eof);
                self.eof = true;
            }
            let mut bytes = vec![];
            let mut consume = 0;
            if err.is_none() {
                self.quoted = match buff.peek() {
                    Some(c) => {
                        if **c == QUOTE {
                            consume += 1;
                            true
                        } else {
                            false
                        }
                    }
                    _ => false
                };
                match buff.peek() {
                    Some(s) => {
                        match **s {
                            NEWLINE | LINEFEED => {
                                consume += 1;
                                err = Some(ParseError::Eol);
                            },
                             _ => {}
                        }
                    }, _ => {}
                }
                for _ in 0..consume {
                    let _ = buff.next();
                }
                // consume = 0;
                if let Some(&&NEWLINE) = buff.peek() {
                    consume += 1;
                    err = Some(ParseError::Eol);
                }
                if err.is_none() {
                    // for _ in 0..consume {
                    //     let _ = buff.next();
                    // }

                    while let Some(byte) = buff.next() {
                        if self.quoted {
                            if *byte == QUOTE {
                                self.quoted = false;
                                consume += 1;

                                continue;
                            }
                            consume += 1;
                            bytes.push(*byte);
                        } else {
                            if *byte == COMMA {
                                consume += 1;
                                break
                            }
                            if *byte == NEWLINE || *byte == LINEFEED {
                                // consume += 1;
                                break
                            }
                            bytes.push(*byte);
                            consume += 1;
                        }
                    }
                }
            }
            (consume, bytes, err)
        };
        self.reader.consume(consumed);
        if let Some(e) = err {
            return Err(e);
        }
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }
}

impl <T: BufRead>Iterator for Parser<T> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Vec<String>> {
        if self.eof {
            return None;
        }
        let mut row: Vec<String>= vec![];
        if let Ok(first) = self.next_part() {
            row.push(first);
        } else {
            return None;
        }
        while let Ok(part) = self.next_part() {
            row.push(part);
        };
        Some(row)
    }
}
