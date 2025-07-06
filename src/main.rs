fn main() {
    println!("Hello, world!");
    let x = 10;
    println!("{}", x);
}

fn zap() -> i32 {
    return 1;
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::{complete::{tag, take_while1}, take},
        character::complete::{char, space1, digit1, multispace0},
        combinator::{map, map_res},
        multi::separated_list0,
        number::double,
        sequence::{delimited, preceded, terminated},
        IResult,
        Parser
    };

    #[derive(Debug, PartialEq)]
    // this is a type that is all the options a parser can return
    enum Value<'a> {
        Str(&'a str),
        Array(Vec<Value<'a>>),
    }

    fn tag_basic(input: &str) -> IResult<&str, &str> {
        //  note that this is really creating a function, the parser for abc
        //  vvvvv 
        //         which is then called here, returning an IResult<&str, &str>
        //         vvvvv
        tag("abc")(input)
    }

    fn parse_key(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c.is_alphanumeric())(input)
    }

    fn parse_digits(input: &str) -> IResult<&str, &str> {
        digit1(input)
    }
    
    fn parse_string(input: &str) -> IResult<&str, &str> {
        delimited(
            char('"'),
            take_while1(|c| c != '"'),
            char('"'),
        ).parse(input)
    }

    fn parse_string_new(input: &str) -> IResult<&str, Value> {
        map(
            delimited(
                char('"'),
                take_while1(|c| c != '"'),
                char('"'),
            ),
            Value::Str
        ).parse(input)
    }

    fn parse_array_basic(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list0(tag(" "), tag("abc")).parse(input)
    }

    fn parse_array(input: &str) -> IResult<&str, Vec<&str>> {
        delimited(
            tag("["),
            separated_list0(tag(" "), parse_string
            ),
            tag("]"),
        ).parse(input)
    }

    fn parse_array_new(input: &str) -> IResult<&str, Value> {
        let parse_value = alt((parse_string_new, parse_array_new));
        map(
            delimited(tag("["), separated_list0(tag(" "), parse_value), tag("]")),
            Value::Array,
        ).parse(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tag_basic_success() {
            let result = tag_basic("abc123");
            assert_eq!(result, Ok(("123", "abc")));
        }

        #[test]
        fn test_tag_basic_failure() {
            let result = tag_basic("xyz");
            assert!(result.is_err());
        }

        #[test]
        fn test_parse_digits() {
            assert_eq!(parse_digits("123abc"), Ok(("abc", "123")));
            // note this only parses integers
            assert_eq!(parse_digits("151.241"), Ok((".241", "151")));
        }

        #[test]
        fn test_parse_string() {
            let result = parse_string("xyz");
            assert!(result.is_err());
            let result = parse_string("\"xyz\"");
            assert_eq!(result, Ok(("", "xyz")));
            let result = parse_string("\"fizzbuzz\"zpzp");
            assert_eq!(result, Ok(("zpzp", "fizzbuzz")));
        }

        #[test]
        fn test_parse_string_new() {
            use Value::Str;
            let result = parse_string_new("xyz");
            assert!(result.is_err());
            let result = parse_string_new("\"xyz\"");
            assert_eq!(result, Ok(("", Str("xyz"))));
            let result = parse_string_new("\"fizzbuzz\"zpzp");
            assert_eq!(result, Ok(("zpzp", Str("fizzbuzz"))));
        }

        #[test]
        fn test_parse_array_basic() {
            let result = parse_array_basic("abc abc abc");
            assert_eq!(result, Ok(("", vec!["abc", "abc", "abc"])));
        }

        #[test]
        fn test_parse_array() {
            let result = parse_array("[\"abc\" \"abc\" \"abc\"]");
            assert_eq!(result, Ok(("", vec!["abc", "abc", "abc"])));
            let result = parse_array("[\"abc\" \"beeep\" \"foo\"]");
            assert_eq!(result, Ok(("", vec!["abc", "beeep", "foo"])));
        }

        #[test]
        fn test_parse_array_new() {
            use Value::Str;
            use Value::Array;
            let result = parse_array_new("[\"abc\" \"abc\" \"abc\"]");
            assert_eq!(result, Ok(("", Array(vec![Str("abc"), Str("abc"), Str("abc")]))));
            let result = parse_array_new("[\"abc\" \"beeep\" \"foo\"]");
            assert_eq!(result, Ok(("", Array(vec![Str("abc"), Str("beeep"), Str("foo")]))));
            let result = parse_array_new("[\"abc\" \"beeep\" [\"wee\"] \"foo\"]");
            assert_eq!(result, Ok(("", Array(vec![Str("abc"), Str("beeep"), Array(vec![Str("wee")]), Str("foo")]))));
        }
    }
}

#[cfg(test)]
mod test {
    use super::zap;

    #[test]
    fn test_trivial() {
        assert_eq!(zap(), 1)
    }
}
