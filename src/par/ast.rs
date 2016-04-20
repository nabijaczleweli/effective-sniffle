#[derive(Debug, Clone)]
pub enum ASTNode {
	Expression(Primitive),
	Value(Primitive),
}

#[derive(Debug, Clone)]
pub enum Primitive {
	Number(f32),
	String(String),
}
