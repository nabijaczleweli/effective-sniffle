#[derive(Debug, Clone)]
pub enum ASTNode {
	Expression(Primitive),
	Value(Primitive),
}

#[derive(Debug, Clone)]
pub enum Primitive {
	Number(u64),
	String(String),
	Null,
}
