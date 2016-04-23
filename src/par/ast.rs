use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Expression(Expr),
    Value(Primitive),
    Null,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Bi {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Simple(Primitive),
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    BOr,
    BAnd,
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Number(u64),
    String(String),
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            "|" => Ok(Op::BOr),
            "&" => Ok(Op::BAnd),
            _ => Err(()),
        }
    }
}
