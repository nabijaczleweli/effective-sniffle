use self::super::{ASTNode, Primitive, Expr, Op};
use std::str::{self, FromStr};
use nom::{multispace, digit};

named!(
    pub program(&[u8]) -> Vec<ASTNode>,
    chain!(
        lines:
            many0!(
                single_line
            ) ~
        multispace? ,

        || { lines }
    )
);

named!(
    single_line(&[u8]) -> ASTNode,
    chain!(
        multispace? ~
        l: line ~
        tag!(";") ,

        || { l }
    )
);

named!(
    line(&[u8]) -> ASTNode,
    alt!(
        ast_value |
        expression |
        null
    )
);

named!(
    null(&[u8]) -> ASTNode,
    map!(
        take!(0),
        |_| ASTNode::Null
    )
);

named!(
    ast_value(&[u8]) -> ASTNode,
    map!(
        val,
        |v| ASTNode::Value(v)
    )
);

named!(
    expression(&[u8]) -> ASTNode,
    map!(
        any_expr,
        |v| ASTNode::Expression(v)
    )
);

named!(
    any_expr(&[u8]) -> Expr,
    alt!(
        bi_expr |
        val_expr
    )
);

named!(
    val_expr(&[u8]) -> Expr,
    chain!(
        lhs: any_expr ~
        op: operator ~
        rhs: any_expr,

        || {
            Expr::Bi {
                lhs: Box::new(lhs),
                op: op,
                rhs: Box::new(rhs),
            }
        }
    )
);

named!(
    bi_expr(&[u8]) -> Expr,
    map!(
        val,
        |v| Expr::Simple(v)
    )
);

named!(
    operator(&[u8]) -> Op,
    map!(
        alt!(
            tag!("+") |
            tag!("-") |
            tag!("*") |
            tag!("/") |
            tag!("|") |
            tag!("&")
        ),
        |o| Op::from_str(str::from_utf8(o).unwrap()).unwrap()
    )
);

named!(
    val(&[u8]) -> Primitive,
    alt!(
        string |
        number
    )
);

named!(
    string(&[u8]) -> Primitive,
    map!(
        delimited!(
            char!('"'),
            is_not!("\""),
            char!('"')
        ),
        |s| Primitive::String(str::from_utf8(s).unwrap().to_string())
    )
);

named!(
    number(&[u8]) -> Primitive,
    map!(
        digit,
        |n| Primitive::Number(str::parse(str::from_utf8(n).unwrap()).unwrap())
    )
);
