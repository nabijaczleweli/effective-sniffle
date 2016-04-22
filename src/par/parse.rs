use peggler::{ParseResult, ParseError};
use self::super::{ASTNode, Primitive};
use std::str;

rule!(program:Vec<ASTNode> =
        lines: single_line*
        whitespace
        => { lines.into_iter().collect() });

rule!(single_line:ASTNode =
        whitespace
        value: expr
        [";"]
        => { value });

rule!(expr:ASTNode =
        value: val
        => { ASTNode::Value(value) });

rule!(val:Primitive =
        value: (string / number)
        => { value });

rule!(string:Primitive =
        ["\""]
        s: alnum*
        ["\""]
        => { Primitive::String(s.into_iter().collect()) });

rule!(number:Primitive =
        num: digit*
        => {
            let num: String = num.into_iter().collect();
            match &num[..] {
                "" => Primitive::Null,
                num => Primitive::Number(str::parse(num).unwrap()),
            }
        });

rule!(whitespace:() =
        ([" "] / ["\t"] / ["\r"]/ ["\n"])*
        => { () });

rule!(digit:char =
        d: (["0"] / ["1"] / ["2"] / ["3"] / ["4"] / ["5"] / ["6"] / ["7"] / ["8"] / ["9"])
        => { d.chars().next().unwrap() });

fn alnum(input: &str) -> ParseResult<char> {
    let mut char_indices = input.char_indices();
    match char_indices.next() {
        Some((_, c)) => match char_indices.next() {
            Some((index, _)) =>
                match c {
                    '"' => Err(ParseError),
                    _ => Ok((c, &input[index..])),
                },
            None => Ok((c, &""[..])),
        },
        None => Err(ParseError),
    }
}

pub fn parse(bytes: &[u8]) -> Result<Vec<ASTNode>, String> {
    let source = str::from_utf8(bytes).unwrap();
    match program(source) {
        Ok((nodes, "")) => Ok(nodes),
        Ok((_, unparsed)) => Err(format!("Parsing failed, no rules expected {:?}", &unparsed[0..unparsed.find("\n").unwrap_or(unparsed.len())])),
        Err(_) => Err("General parse error".to_string()),
    }
}
