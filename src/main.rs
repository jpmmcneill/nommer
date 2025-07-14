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
        bytes::{complete::{tag, take_while1}, take, take_until},
        character::complete::{char, space1, digit1, multispace0},
        combinator::{map, map_res},
        multi::separated_list0,
        number::double,
        sequence::{delimited, preceded, terminated, separated_pair},
        IResult,
        Parser
    };
    use std::collections::HashMap;

    #[derive(Debug, PartialEq)]
    // this is a type that is all the options a parser can return
    enum Value<'a> {
        Str(&'a str),
        Array(Vec<Value<'a>>),
        Map(HashMap<&'a str, Value<'a>>)
    }

    fn tag_basic(input: &str) -> IResult<&str, &str> {
        //  note that this is really creating a function, the parser for abc
        //  vvvvv 
        //         which is then called here, returning an IResult<&str, &str>
        //         vvvvv
        tag("abc")(input)
    }

    fn parse_digits(input: &str) -> IResult<&str, &str> {
        digit1(input)
    }

    fn parse_string(input: &str) -> IResult<&str, Value> {
        map(
            delimited(
                char('"'),
                take_while1(|c| c != '"'),
                char('"'),
            ),
            Value::Str
        ).parse(input)
    }

    fn parse_array(input: &str) -> IResult<&str, Value> {
        let parse_value = alt((parse_string, parse_array));
        map(
            delimited(tag("["), separated_list0(tag(" "), parse_value), tag("]")),
            Value::Array,
        ).parse(input)
    }

    fn parse_key(input: &str) -> IResult<&str, &str> {
        take_until("=").parse(input)
    }

    fn parse_map(input: &str) -> IResult<&str, Value> {
        map(
            delimited(
                char('{'),
                parse_key_value,
                char('}')
            ),
            Value::Map
        ).parse(input)
    }

    fn parse_value(input: &str) -> IResult<&str, Value> {
        alt((
            parse_string,
            parse_array,
            parse_map,
        )).parse(input)
    }

    fn parse_key_value(input: &str) -> IResult<&str, HashMap<&str, Value>> {
        let kv_parser = separated_pair(
            parse_key,
            char('='),
            parse_value,
        );

        map(
            separated_list0(multispace0, kv_parser),
            |pairs| pairs.into_iter().collect::<HashMap<_, _>>()
        ).parse(input)
    }

    pub fn parse_message(input: &str) -> IResult<&str, Value> {
        delimited(
            char('{'),
            parse_key_value,
            char('}')
        ).parse(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use Value::{Str, Array, Map};

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
            use Value::Str;
            let result = parse_string("xyz");
            assert!(result.is_err());
            let result = parse_string("\"xyz\"");
            assert_eq!(result, Ok(("", Str("xyz"))));
            let result = parse_string("\"fizzbuzz\"zpzp");
            assert_eq!(result, Ok(("zpzp", Str("fizzbuzz"))));
        }

        #[test]
        fn test_parse_array() {
            // this guy can parse a nested array!
            let result = parse_array("[\"abc\" \"abc\" \"abc\"]");
            assert_eq!(result, Ok(("", Array(vec![Str("abc"), Str("abc"), Str("abc")]))));
            let result = parse_array("[\"abc\" \"beeep\" \"foo\"]");
            assert_eq!(result, Ok(("", Array(vec![Str("abc"), Str("beeep"), Str("foo")]))));
            let result = parse_array("[\"abc\" \"beeep\" [\"wee\"] \"foo\"]");
            assert_eq!(result, Ok(("", Array(vec![Str("abc"), Str("beeep"), Array(vec![Str("wee")]), Str("foo")]))));

            let result = parse_array(r#"["abc" "beeep" ["fuzz" ["wee" "smat"] "food"] "foo"]"#);
            assert_eq!(
                result,
                Ok((
                    "",
                    Value::Array(vec![
                        Value::Str("abc"),
                        Value::Str("beeep"),
                        Value::Array(vec![
                            Value::Str("fuzz"),
                            Value::Array(vec![
                                Value::Str("wee"),
                                Value::Str("smat")
                            ]),
                            Value::Str("food"),
                        ]),
                        Value::Str("foo")
                    ])
                ))
            );

        #[test]
        fn test_parse_key() {
            let input = "myKey = \"value\"";
            let (rest, key) = parse_key(input).unwrap();
            assert_eq!(key, "myKey");
            assert!(rest.starts_with(" = \"value\""));
        }

        #[test]
        fn test_parse_value() {
            // test string
            let input = r#""hello""#;
            let (rest, value) = parse_value(input).unwrap();
            assert_eq!(value, Str("hello"));
            assert_eq!(rest, "");

            // test array
            let input = r#"["a" "b" "c"]"#;
            let (rest, value) = parse_value(input).unwrap();
            assert_eq!(
                value,
                Array(vec![Str("a"), Str("b"), Str("c")])
            );
            assert_eq!(rest, "");

            let input = r#"{"bar"="baz" nest=["a" "b"]}"#;
            let (rest, value) = parse_value(input).unwrap();
            assert_eq!(rest, "");
            assert_eq!(
                value,
                Map(
                    vec![
                        ("bar", Str("baz")),
                        ("nest", Array(vec![Str("a"), Str("b")]))
                    ].into_iter().collect()
                )
            );
        }
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
