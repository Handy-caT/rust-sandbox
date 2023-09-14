use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1};
use nom::combinator::value;
use nom::Err::Error;
use nom::IResult;
use nom::sequence::tuple;
use regex::Regex;

fn main() {
    println!("Implement me!");
}


trait Parser {
    fn parse(&self, input: &str) -> (Option<Sign>, Option<Width>, Option<Precision>);
}

#[derive(Debug, PartialEq)]
struct Width(pub Count);

#[derive(Debug, PartialEq, Clone)]
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

struct NomParser;

impl NomParser {
    fn new() -> Self {
        Self {}
    }

    fn parse_sign<'a>(&self, input: &'a str) -> IResult<&'a str, Sign> {
        alt(
            (
                value(Sign::Plus, tag("+")),
                value(Sign::Minus, tag("-")),
            )
        )(input)
    }

    fn parse_align<'a>(&self, input: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
        tuple(
            (
                alt(
                    (
                        alphanumeric1,
                        tag(""),
                    )
                ),
                alt(
                    (
                        tag("<"),
                        tag(">"),
                        tag("^"),
                    )
                )
            )
        )(input)
    }

    fn parse_grid<'a>(&self, input: &'a str) -> IResult<&'a str, &'a str> {
        tag("#")(input)
    }

    fn parse_zero<'a>(&self, input: &'a str) -> IResult<&'a str, &'a str> {
        tag("0")(input)
    }

    fn parse_width<'a>(&self, input: &'a str) -> IResult<&'a str, Width> {
        let res = self.parse_count(input);
        match res {
            Ok((input, output)) => {
                Ok((input, Width(output)))
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    fn parse_precision<'a>(&self, input: &'a str) -> IResult<&'a str, Precision> {
        let res = tag(".")(input);
        match res {
            Ok((input, _)) => {
                let res = self.parse_count(input);
                match res {
                    Ok((input, output)) => {
                        Ok((input, Precision::Count(output)))
                    }
                    Err(err) => {
                        let res = tag("*")(input);
                        match res {
                            Ok((input, _)) => {
                                Ok((input, Precision::Asterisk))
                            }
                            Err(err) => {
                                Err(err)
                            }
                        }
                    }
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    fn parse_count<'a>(&self, input: &'a str) -> IResult<&'a str, Count> {
        let res = self.parse_parameter(input);
        match res {
            Ok((input, output)) => {
                if input.starts_with('$') {
                    let res = input.strip_prefix('$').unwrap();
                    Ok((res, Count::Parameter(output)))
                } else {
                    let res = digit1(input)?;
                    let integer = res.1.parse::<usize>().unwrap();
                    Ok((res.0, Count::Integer(Integer(integer))))
                }
            }
            Err(err) => {
                let res = digit1(input)?;
                let integer = res.1.parse::<usize>().unwrap();
                Ok((res.0, Count::Integer(Integer(integer))))
            }
        }
    }

    fn parse_parameter<'a>(&self, input: &'a str) -> IResult<&'a str, Argument> {
        let res = self.parse_argument(input);
        match res {
            Ok((input, output)) => {
                if input.starts_with("$") {
                    Ok((input, output))
                } else {
                    Err(Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    fn parse_argument<'a>(&self, input: &'a str) -> IResult<&'a str, Argument> {
        let res: IResult<&'a str, &'a str, ()> = digit1(input);
        match res {
            Ok((input, output)) => {
                let integer = output.parse::<usize>().unwrap();
                Ok((input, Argument::Integer(Integer(integer))))
            }
            Err(_) => {
                let res = alphanumeric1(input)?;

                let identifier = res.1.to_string();
                Ok((res.0, Argument::Identifier(Identifier(identifier))))
            }
        }
    }
}

impl Parser for NomParser {
    fn parse(&self, input: &str) -> (Option<Sign>, Option<Width>, Option<Precision>) {
        let mut input = input;
        let mut sing = None;
        let mut width = None;
        let mut precision = None;

        let align_res = self.parse_align(input);
        if let Ok(..) = align_res {
            input = align_res.unwrap().0;
        }

        let sign_res = self.parse_sign(input);
        if let Ok(..) = sign_res {
            let (input_next, output) = sign_res.unwrap();
            sing = Some(output);
            input = input_next;
        }

        let grid_res = self.parse_grid(input);
        if let Ok(..) = grid_res {
            input = grid_res.unwrap().0;
        }

        let zero_res = self.parse_zero(input);
        if let Ok(..) = zero_res {
            input = zero_res.unwrap().0;
        }

        let width_res = self.parse_width(input);
        if let Ok(..) = width_res {
            let (input_next, output) = width_res.unwrap();
            width = Some(output);
            input = input_next;
        }

        let precision_res = self.parse_precision(input);
        if let Ok(..) = precision_res {
            let (input_next, output) = precision_res.unwrap();
            precision = Some(output);
            input = input_next;
        }

        (sing, width, precision)
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
        let parsers: Vec<Box<dyn Parser>> = vec![
            Box::new(RegexParser::new()),
            Box::new(NomParser::new()),
        ];

        for (input, expected) in [
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None)
        ] {
            for parser in parsers.iter() {
                let (sign, ..) = parser.parse(input);
                assert_eq!(sign, expected);
            }
        }
    }

    #[test]
    fn parses_width() {
        let parsers: Vec<Box<dyn Parser>> = vec![
            Box::new(RegexParser::new()),
            Box::new(NomParser::new()),
        ];
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Width(Count::Integer(Integer(8))))),
            (">+8.*", Some(Width(Count::Integer(Integer(8))))),
            ("-.1$x", None),
            ("a^#043.8?", Some(Width(Count::Integer(Integer(43))))),
            ("-1$.1$x", Some(Width(Count::Parameter(Argument::Integer(Integer(1)))))),
            ("-a1$.1$x", Some(Width(Count::Parameter(Argument::Identifier(Identifier("a1".to_string())))))),
        ] {
            for parser in parsers.iter() {
                let (_, width, ..) = parser.parse(input);
                assert_eq!(width, expected);
            }
        }
    }


    #[test]
    fn parses_precision() {
        let parsers: Vec<Box<dyn Parser>> = vec![
            Box::new(RegexParser::new()),
            Box::new(NomParser::new()),
        ];
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Count(Count::Parameter(Argument::Integer(Integer(1)))))),
            ("a^#043.8?", Some(Precision::Count(Count::Integer(Integer(8))))),
            ("-1$.1$x", Some(Precision::Count(Count::Parameter(Argument::Integer(Integer(1)))))),
            ("-1$.a1$x", Some(Precision::Count(Count::Parameter(Argument::Identifier(Identifier("a1".to_string()))))))
        ] {
            for parser in parsers.iter() {
                let (_, _, precision) = parser.parse(input);
                assert_eq!(precision, expected);
            }
        }
    }

    #[test]
    fn test_parse_sign_nom() {
        let input = "+";
        let parser = NomParser::new();
        let result = parser.parse_sign(input);

        assert_eq!(result.unwrap(), ("", Sign::Plus));

        let input = "-";
        let result = parser.parse_sign(input);

        assert_eq!(result.unwrap(), ("", Sign::Minus));
    }

    #[test]
    fn test_parse_align_nom() {
        let input = "a^#043.8?";
        let parser = NomParser::new();
        let result = parser.parse_align(input);

        assert_eq!(result.unwrap(), ("#043.8?", ("a", "^")));
    }

    #[test]
    fn test_parse_grid_nom() {
        let input = "#043.8?";
        let parser = NomParser::new();
        let result = parser.parse_grid(input);

        assert_eq!(result.unwrap(), ("043.8?", "#"));
    }

    #[test]
    fn test_parse_nom_full() {
        let input = "a^+#043.8?";
        let parser = NomParser::new();

        let result = parser.parse_align(input);
        assert!(result.is_ok());
        let (input, align) = result.unwrap();
        assert_eq!(align, ("a", "^"));

        let result = parser.parse_sign(input);
        assert!(result.is_ok());
        let (input, sign) = result.unwrap();
        assert_eq!(sign, Sign::Plus);

        let result = parser.parse_grid(input);
        assert!(result.is_ok());
        let (input, grid) = result.unwrap();
        assert_eq!(grid, "#");

        let result = parser.parse_zero(input);
        assert!(result.is_ok());
        let (input, zero) = result.unwrap();
        assert_eq!(zero, "0");

        let result = parser.parse_width(input);
        assert!(result.is_ok());
        let (input, width) = result.unwrap();
        assert_eq!(width, Width(Count::Integer(Integer(43))));

        let result = parser.parse_precision(input);
        assert!(result.is_ok());
        let (input, precision) = result.unwrap();
        assert_eq!(precision, Precision::Count(Count::Integer(Integer(8))));
    }
}
