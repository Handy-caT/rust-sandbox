use regex::Regex;

fn main() {
    println!("Implement me!");
}


trait Parser {
    fn parse(&self, input: &str) -> (Option<Sign>, Option<Width>, Option<Precision>);
}

#[derive(Debug, PartialEq)]
struct Width(pub Count);

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Count(Count),
    Asterisk,
}

#[derive(Debug, PartialEq)]
enum Count {
    Parameter(Argument),
    Integer(Integer)
}

#[derive(Debug, PartialEq)]
enum Argument {
    Integer(Integer),
    Identifier(Identifier),
}

#[derive(Debug, PartialEq)]
struct Integer(pub usize);
#[derive(Debug, PartialEq)]
struct Identifier(pub String);


struct RegexParser {
    regex_full: Regex,
    regex_integer: Regex,
    regex_identifier: Regex,
    regex_argument: Regex,
}


impl RegexParser {
    fn new() -> Self {
        let regex_full = Regex::new(r"(?P<align>.?[<>^])?(?P<sign>[+\-])?(?P<grid>#)?(?P<zero>0)?(?P<width>[0-9a-zA-Z_]+\$?)?(?P<precision>\.[0-9a-zA-Z_*]+\$?)?(?P<type>[0-9a-zA-Z_?])?").unwrap();
        let regex_integer = Regex::new(r"[0-9]+").unwrap();
        let regex_identifier = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
        let regex_argument = Regex::new(r"[a-zA-Z0-9_]+\$").unwrap();

        RegexParser {
            regex_full,
            regex_integer,
            regex_identifier,
            regex_argument,
        }
    }

    fn parse_integer(&self, input: Option<String>) -> Option<Integer> {
        match input {
            None => {None}
            Some(string) => {
                match self.regex_integer.captures(&string) {
                    None => {None}
                    Some(captures) => {
                        let integer = captures.get(0).unwrap().as_str().parse::<usize>().unwrap();
                        Some(Integer(integer))
                    }
                }
            }
        }
    }

    fn parse_identifier(&self, input: Option<String>) -> Option<Identifier> {
        match input {
            None => {None}
            Some(string) => {
                match self.regex_identifier.captures(&string) {
                    None => {None}
                    Some(captures) => {
                        let identifier = captures.get(0).unwrap().as_str().to_string();
                        Some(Identifier(identifier))
                    }
                }
            }
        }
    }

    fn parse_argument(&self, input: Option<String>) -> Option<Argument> {
        match input {
            None => {None}
            Some(string) => {
                match self.regex_argument.captures(&string) {
                    None => {None}
                    Some(captures) => {
                        let identifier = self.parse_identifier(Some(string.clone()));
                        match identifier {
                            None => {
                                let integer = self.parse_integer(Some(string)).unwrap();
                                Some(Argument::Integer(integer))
                            }
                            Some(identifier) => {
                                Some(Argument::Identifier(identifier))
                            }
                        }
                    }
                }
            }
        }
    }

    fn parse_count(&self, input: Option<String>) -> Option<Count> {
        match input {
            None => {None}
            Some(string) => {
                let argument = self.parse_argument(Some(string.clone()));
                match argument {
                    None => {
                        let integer = self.parse_integer(Some(string)).unwrap();
                        Some(Count::Integer(integer))
                    }
                    Some(argument) => {
                        Some(Count::Parameter(argument))
                    }
                }
            }
        }
    }

    fn parse_sign(&self, input: Option<String>) -> Option<Sign> {
        match input {
            None => {None}
            Some(string) => {
                match string.as_str() {
                    "+" => {Some(Sign::Plus)}
                    "-" => {Some(Sign::Minus)}
                    _ => {None}
                }
            }
        }
    }

    fn parse_width(&self, input: Option<String>) -> Option<Width> {
        match input {
            None => {None}
            Some(string) => {
                let count = self.parse_count(Some(string.clone())).unwrap();
                Some(Width(count))
            }
        }
    }

    fn parse_precision(&self, input: Option<String>) -> Option<Precision> {
        match input {
            None => {None}
            Some(string) => {
                match string.as_str() {
                     ".*" => {Some(Precision::Asterisk)}
                        _ => {
                            let count = self.parse_count(Some(string.clone())).unwrap();
                            Some(Precision::Count(count))
                        }
                }
            }
        }
    }
}

impl Parser for RegexParser {
    fn parse(&self, input: &str) -> (Option<Sign>, Option<Width>, Option<Precision>) {
        let result = self.regex_full.captures(input).unwrap();

        let sign = self.parse_sign(result.name("sign").map(|m| m.as_str().to_string()));
        let width = self.parse_width(result.name("width").map(|m| m.as_str().to_string()));
        let precision = self.parse_precision(result.name("precision").map(|m| m.as_str().to_string()));

        (sign, width, precision)
    }
}


#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn test_parse_regex_full() {
        let input = "a^#043.8?";
        let parser = RegexParser::new();
        let result = parser.regex_full.captures(input).unwrap();

        assert_eq!(result.name("align").unwrap().as_str(), "a^");
        assert_eq!(result.name("grid").unwrap().as_str(), "#");
        assert!(result.name("sign").is_none());
        assert_eq!(result.name("width").unwrap().as_str(), "43");
        assert_eq!(result.name("precision").unwrap().as_str(), ".8");
        assert_eq!(result.name("type").unwrap().as_str(), "?");
    }

    #[test]
    fn test_parse_regex_integer() {
        let input = "43";
        let parser = RegexParser::new();
        let result = parser.parse_integer(Some(input.to_string()));

        assert_eq!(result.unwrap(), Integer(43));
    }

    #[test]
    fn test_parse_regex_identifier() {
        let input = "a43";
        let parser = RegexParser::new();
        let result = parser.parse_identifier(Some(input.to_string()));

        assert_eq!(result.unwrap(), Identifier("a43".to_string()));
    }

    #[test]
    fn test_parse_regex_argument() {
        let input = "a43$";
        let parser = RegexParser::new();
        let result = parser.parse_argument(Some(input.to_string()));

        assert_eq!(result.unwrap(), Argument::Identifier(Identifier("a43".to_string())));

        let input = "43$";
        let result = parser.parse_argument(Some(input.to_string()));

        assert_eq!(result.unwrap(), Argument::Integer(Integer(43)));
    }

    #[test]
    fn test_parse_regex_count() {
        let input = "a43$";
        let parser = RegexParser::new();
        let result = parser.parse_count(Some(input.to_string()));

        assert_eq!(result.unwrap(), Count::Parameter(Argument::Identifier(Identifier("a43".to_string()))));

        let input = "43$";
        let result = parser.parse_count(Some(input.to_string()));

        assert_eq!(result.unwrap(), Count::Parameter(Argument::Integer(Integer(43))));

        let input = "43";
        let result = parser.parse_count(Some(input.to_string()));

        assert_eq!(result.unwrap(), Count::Integer(Integer(43)));
    }

    #[test]
    fn test_parse_regex_sign() {
        let input = "+";
        let parser = RegexParser::new();
        let result = parser.parse_sign(Some(input.to_string()));

        assert_eq!(result.unwrap(), Sign::Plus);

        let input = "-";
        let result = parser.parse_sign(Some(input.to_string()));

        assert_eq!(result.unwrap(), Sign::Minus);
    }

    #[test]
    fn test_parse_regex_precision() {
        let input = ".*";
        let parser = RegexParser::new();
        let result = parser.parse_precision(Some(input.to_string()));

        assert_eq!(result.unwrap(), Precision::Asterisk);

        let input = ".43$";
        let result = parser.parse_precision(Some(input.to_string()));

        assert_eq!(result.unwrap(), Precision::Count(Count::Parameter(Argument::Integer(Integer(43)))));

        let input = ".a43$";
        let result = parser.parse_precision(Some(input.to_string()));

        assert_eq!(result.unwrap(), Precision::Count(Count::Parameter(Argument::Identifier(Identifier("a43".to_string())))));

        let input = ".43";
        let result = parser.parse_precision(Some(input.to_string()));

        assert_eq!(result.unwrap(), Precision::Count(Count::Integer(Integer(43))));
    }

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let parser = RegexParser::new();
            let (sign, ..) = parser.parse(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Width(Count::Integer(Integer(8))))),
            (">+8.*", Some(Width(Count::Integer(Integer(8))))),
            ("-.1$x", None),
            ("a^#043.8?", Some(Width(Count::Integer(Integer(43))))),
            ("-1$.1$x", Some(Width(Count::Parameter(Argument::Integer(Integer(1)))))),
            ("-a1$.1$x", Some(Width(Count::Parameter(Argument::Identifier(Identifier("a1".to_string())))))),
        ] {
            let parser = RegexParser::new();
            let (_, width, _) = parser.parse(input);
            assert_eq!(width, expected);
        }
    }


    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Count(Count::Parameter(Argument::Integer(Integer(1)))))),
            ("a^#043.8?", Some(Precision::Count(Count::Integer(Integer(8))))),
            ("-1$.1$x", Some(Precision::Count(Count::Parameter(Argument::Integer(Integer(1)))))),
            ("-1$.a1$x", Some(Precision::Count(Count::Parameter(Argument::Identifier(Identifier("a1".to_string()))))))
        ] {
            let parser = RegexParser::new();
            let (_, _, precision) = parser.parse(input);
            assert_eq!(precision, expected);
        }
    }
}
