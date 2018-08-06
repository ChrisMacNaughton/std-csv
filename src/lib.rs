use std::io::BufRead;

mod parser;

pub use parser::Parser;

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::str;
    use super::*;
    use std::io::Cursor;

    mod csv_spectrum {
        use super::*;

        fn assert_parses_file<T: PartialEq<String> + Debug>(bytes: &[u8], expected: Vec<Vec<T>>) {
            assert_should_parse(str::from_utf8(bytes).unwrap(), expected);
        }

        #[test]
        fn comma_in_quotes() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/comma_in_quotes.csv"), vec![
                vec!["first","last","address","city","zip"],
                vec!["John","Doe","120 any st.","Anytown, WW", "08123"],
            ]);
        }

        #[test]
        fn empty() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/empty.csv"), vec![
                vec!["a", "b", "c"],
                vec!["1", "", ""],
                vec!["2", "3", "4"],
            ]);
        }

        #[test]
        fn empty_crlf() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/empty_crlf.csv"), vec![
                vec!["a", "b", "c"],
                vec!["1", "", ""],
                vec!["2", "3", "4"],
            ]);
        }

        #[test]
        #[ignore]
        fn escaped_quotes() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/escaped_quotes.csv"), vec![
                vec!["a", "b"],
                vec!["1", r#"ha "ha" ha"#],
                vec!["3", "4"],
            ]);
        }

        #[test]
        #[ignore]
        fn json() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/json.csv"), vec![
                vec!["key","val"],
                vec!["1", "{\"type\": \"Point\", \"coordinates\": [102.0, 0.5]}"],
            ]);

        }

        #[test]
        fn newlines() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/newlines.csv"), vec![
                vec!["a","b","c"],
                vec!["1","2","3"],
                vec!["Once upon \na time","5","6"],
                vec!["7","8","9"],
            ]);
        }

        #[test]
        fn newlines_crlf() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/newlines_crlf.csv"), vec![
                vec!["a","b","c"],
                vec!["1","2","3"],
                vec!["Once upon \r\na time","5","6"],
                vec!["7","8","9"],
            ]);
        }

        #[test]
        #[ignore]
        fn quotes_and_newlines() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/quotes_and_newlines.csv"), vec![
                vec!["a","b"],
                vec!["1","ha \n\"ha\" \nha"],
                vec!["3", "4"],
            ]);
        }

        #[test]
        fn simple() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/simple.csv"), vec![
                vec!["a","b","c"],
                vec!["1","2","3"],
            ]);
        }

        #[test]
        fn simple_crlf() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/simple_crlf.csv"), vec![
                vec!["a","b","c"],
                vec!["1","2","3"],
            ]);
        }

        #[test]
        fn utf8() {
            assert_parses_file(include_bytes!("../tests/csv-spectrum/csvs/utf8.csv"), vec![
                vec!["a","b","c"],
                vec!["1","2","3"],
                vec!["4","5","ʤ"],
            ]);
        }
    }

    fn assert_should_parse<T: PartialEq<String> + Debug>(input: &str, expected: Vec<Vec<T>>) {
        let mut input = Cursor::new(input);
        assert_eq!(expected, parse(&mut input).unwrap());
    }

    #[test]
    fn it_handles_newline_in_row() {
        assert_should_parse("\"quoted\nnewline\"", vec![vec!["quoted\nnewline"]]);
    }

    #[test]
    fn it_handles_multiple_lines() {
        assert_should_parse(r#"name,dob,email
test1,01-12-1980,test1@example.com
test2,20-2-1970,test2@example.com
"#, vec![
            vec!["name", "dob", "email"],
            vec!["test1", "01-12-1980", "test1@example.com"],
            vec!["test2", "20-2-1970", "test2@example.com"]
        ]);
    }
}

pub fn parse<T>(reader: T) -> Result<Vec<Vec<String>>, ()>
where T: BufRead {
    Ok(Parser::new(reader).collect())
}