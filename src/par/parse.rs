use peggler::ParseResult;
use self::super::{ASTNode, Primitive};
use std::str;


rule!(program:Vec<ASTNode> =
        lines: single_line*
        whitespace
        => { lines.into_iter().collect() });

rule!(single_line:ASTNode =
        whitespace
        value: (digit*)
        [";"]
        => {
            let value: String = value.into_iter().collect();
            match &value[..] {
                "" => ASTNode::Value(Primitive::Null),
                value => ASTNode::Value(Primitive::Number(str::parse(value).unwrap())),
            }
        });

rule!(whitespace:() =
        ([" "] / ["\t"] / ["\r"]/ ["\n"])*
        => { () });

rule!(digit:char =
        d: (["0"] / ["1"] / ["2"] / ["3"] / ["4"] / ["5"] / ["6"] / ["7"] / ["8"] / ["9"])
        => { d.chars().next().unwrap() });

pub fn parse(bytes: &[u8]) -> ParseResult<Vec<ASTNode>> {
    program(str::from_utf8(bytes).unwrap())
}
