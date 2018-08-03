use std::borrow::Cow;
use std::io::BufRead;
use std::io::prelude::*;
use std::str;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_returns_an_unquoted_part() {
        let mut input = Cursor::new("part,another part");
        let expected = "part";
        assert_eq!(expected, take_part(&mut input).unwrap())
    }

    #[test]
    fn it_returns_a_quoted_part() {
        let mut input = Cursor::new("\"quoted,part\",next");
        let expected = "quoted,part";
        assert_eq!(expected, take_part(&mut input).unwrap())
    }

    #[test]
    fn it_handles_newline_in_part() {
        let mut input = Cursor::new("\"quoted\nnewline\"");
        let expected = "quoted\nnewline";
        assert_eq!(expected, take_part(&mut input).unwrap());
    }

    #[test]
    fn it_parses_a_basic_line() {
        let mut input = Cursor::new("a,comma,separated,line");
        let expected = vec!["a", "comma", "separated", "line"];
        assert_eq!(expected, take_line(&mut input).unwrap());
    }

    #[test]
    fn it_handles_newline_in_row() {
        let mut input = Cursor::new("\"quoted\nnewline\"");
        let expected = vec![vec!["quoted\nnewline"]];
        assert_eq!(expected, parse(&mut input).unwrap());
    }

    #[test]
    fn it_handles_multiple_lines() {
        let input_raw = r#"name,dob,email
test1,01-12-1980,test1@example.com
test2,20-2-1970,test2@example.com
"#;
        let mut input = Cursor::new(input_raw);
        let expected = vec![
            vec!["name", "dob", "email"],
            vec!["test1", "01-12-1980", "test1@example.com"],
            vec!["test2", "20-2-1970", "test2@example.com"]
        ];
        assert_eq!(expected, parse(&mut input).unwrap());
    }
}

#[derive(Debug)]
enum TakeError {
    Eol,
    Eof,
}

pub fn parse<'a, T>(mut reader: T) -> Result<Vec<Vec<String>>, ()>
where T: BufRead + 'a {
    let mut lines = vec![];
    while let Ok(line) = take_line(&mut reader) {
        lines.push(line);
    }
    Ok(lines)
}

fn take_line<'a, T>(reader: &mut T) -> Result<Vec<String>, ()>
where T: BufRead + 'a {
    let mut row = vec![];

    loop {

    }
    // while let Ok(part) = take_part(reader) {
    //     row.push(part);
    // }
    // if row.len() == 0 {
    //     let mut buff = reader.fill_buf().map_err(|_| ())?.iter().peekable();
    //     if buff.len() == 0 {
    //         return Err(());
    //     }
    //     return Err(());
    // }
    // Ok(row)
}

fn take_part<'a, T>(reader: &mut T) -> Result<String, TakeError>
where T: BufRead + 'a {
    let (consumed, bytes) = {
        let mut buff = reader.fill_buf().map_err(|_| TakeError::Eof)?.iter().peekable();
        if buff.len() == 0 {
            return Err(TakeError::Eof);
        }
        let mut bytes: Vec<u8> = vec![];
        let mut consume = 0;
        let quote = '"' as u8;
        let comma = ',' as u8;

        let mut quoted = match buff.peek() {
            Some(c) => {
                if **c == '\n' as u8 {
                    return Err(TakeError::Eol)
                }
                if **c == quote {
                    consume += 1;
                    true
                } else {
                    false
                }
            }
            _ => false
        };
        if consume > 0 {
            let _ = buff.next();
        }
        while let Some(byte) = buff.next() {
            if quoted {
                if *byte == quote {
                    quoted = false;
                    consume += 1;
                    continue;
                }
                consume += 1;
                bytes.push(*byte);
            } else {
                if *byte == comma {
                    consume += 1;
                    break
                }
                if *byte == '\n' as u8 {
                    // consume += 1;
                    break
                }
                bytes.push(*byte);
                consume += 1;
            }
        }
        (consume, bytes)
    };
    reader.consume(consumed);
    // Ok(str::from_utf8(&bytes).map_err(|_| ())?)
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}