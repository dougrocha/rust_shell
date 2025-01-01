use anyhow::Result;

#[derive(Clone, Copy)]
pub struct Parser<'a> {
    pub input: &'a str,
    pub position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }
}

impl Iterator for Parser<'_> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.chars();
        let c = chars.next()?;
        self.position += c.len_utf8();

        #[derive(Debug)]
        enum Started {
            SingleQuote,
            DoubleQuote,
            BackSlash,
            Spaces,
            // Group that is surrounded by spaces
            Default,
        }

        let started = match c {
            '\'' => Started::SingleQuote,
            '\"' => Started::DoubleQuote,
            '\\' => Started::BackSlash,
            ' ' => Started::Spaces,
            '\n' => return None,
            _ => Started::Default,
        };

        match started {
            Started::SingleQuote => {
                self.input = &self.input[1..];
                self.position += 1;

                let mut chars = self.input.chars();
                let mut group = String::new();
                let mut prev_escape = false;

                while let Some(c) = chars.next() {
                    match (c, prev_escape) {
                        ('\'', false) => {
                            self.input = chars.as_str();
                            self.position += group.len() + 1;
                            return Some(Ok(group));
                        }
                        ('\'', true) => {
                            group.push(c);
                            prev_escape = false;
                        }
                        ('\\', false) => {
                            prev_escape = true;
                        }

                        (_, true) => {
                            group.push('\\');
                            group.push(c);
                            prev_escape = false;
                        }
                        _ => {
                            group.push(c);
                            prev_escape = false;
                        }
                    }
                }

                self.input = chars.as_str();
                self.position += group.len();

                Some(Err(anyhow::anyhow!(
                    "Unclosed single quote at position {}",
                    self.position
                )))
            }
            Started::DoubleQuote => {
                self.input = &self.input[1..];
                self.position += 1;

                let mut chars = self.input.chars();
                let mut group = String::new();
                let mut prev_escape = false;

                while let Some(c) = chars.next() {
                    match (c, prev_escape) {
                        ('"', false) => {
                            self.input = chars.as_str();
                            self.position += group.len() + 1;
                            return Some(Ok(group));
                        }
                        ('\\', true) | ('"', true) => {
                            group.push(c);
                            prev_escape = false;
                        }
                        ('\\', false) => {
                            prev_escape = true;
                        }

                        (_, true) => {
                            group.push('\\');
                            group.push(c);
                            prev_escape = false;
                        }
                        _ => {
                            group.push(c);
                            prev_escape = false;
                        }
                    }
                }

                self.input = chars.as_str();
                self.position += group.len();

                Some(Err(anyhow::anyhow!(
                    "Unclosed double quote at position {}",
                    self.position
                )))
            }
            Started::BackSlash => {
                let symbol = chars.next().unwrap_or_default();
                self.input = chars.as_str();
                self.position += symbol.len_utf8();

                Some(Ok(symbol.to_string()))
            }
            Started::Default => {
                let stop = self
                    .input
                    .find([' ', '\\', '\"'])
                    .unwrap_or(self.input.len());

                let group = &self.input[..stop];
                self.input = &self.input[stop..];
                self.position = stop;

                Some(Ok(group.to_string()))
            }
            Started::Spaces => {
                let stop = self.input.find(|c| c != ' ').unwrap_or(self.input.len());

                self.input = &self.input[stop..];
                self.position += stop;

                Some(Ok(" ".to_string()))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_quote() {
        let input = "'testing_quote'";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "testing_quote");
    }

    #[test]
    fn test_single_quote_with_space() {
        let input = "'testing      quote'";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "testing      quote");
    }

    #[test]
    fn test_single_quote_with_escaped_space() {
        let input = "'testing\\ space'";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "testing\\ space");
    }

    #[test]
    fn test_multiple_single_quotes() {
        let input = "'testing' 'multiple' 'quotes'";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "testing multiple quotes");
    }

    #[test]
    fn test_single_quote_with_newline() {
        let input = "'line1\nline2'";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "line1\nline2");
    }

    #[test]
    fn test_single_quote_with_special_characters() {
        let input = "'hello!$%^&*()'";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "hello!$%^&*()");
    }

    #[test]
    fn test_empty_single_quote() {
        let input = "''";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "");
    }

    #[test]
    fn test_command_with_single_quotes_and_spaces() {
        let input = "'hello world' > file.txt";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "hello world > file.txt");
    }

    #[test]
    fn test_single_quote_with_leading_trailing_spaces() {
        let input = "' hello world '";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, " hello world ");
    }

    #[test]
    fn test_multiple_single_quote_groups_with_operators() {
        let input = "'hello' > 'world' > file.txt";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "hello > world > file.txt");
    }

    #[test]
    fn test_mixed_single_and_double_quotes() {
        let input = "'hello' \"world\"";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_single_quote_with_escaped_backslash() {
        let input = "'hello\\ world'";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "hello\\ world");
    }

    #[test]
    fn test_single_quote_with_backslash_inside() {
        let input = "'hello\\world'";
        let parser = Parser::new(input);
        let result = parser.filter_map(Result::ok).collect::<String>();

        assert_eq!(result, "hello\\world");
    }

    #[test]
    fn test_unclosed_single_quote() {
        let input = "'hello world";
        let mut parser = Parser::new(input);

        let result = parser.next();

        // Check if an error was returned and validate the error message.
        assert!(
            result.is_some(),
            "Parser should return an error, but got None"
        );
        let error = result
            .unwrap()
            .expect_err("Expected an error for unclosed single quote");

        assert!(
            error.to_string().contains("Unclosed single quote"),
            "Error message did not match expected: {}",
            error
        );
    }

    #[test]
    fn test_unclosed_double_quote() {
        let input = r#""hello world"#;

        let mut parser = Parser::new(input);

        let result = parser.next();

        // Check if an error was returned and validate the error message.
        assert!(
            result.is_some(),
            "Parser should return an error, but got None"
        );
        let error = result
            .unwrap()
            .expect_err("Expected an error for unclosed double quote");

        assert!(
            error.to_string().contains("Unclosed double quote"),
            "Error message did not match expected: {}",
            error
        );
    }
}
