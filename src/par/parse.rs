use self::super::{ASTNode, Primitive};
use parse;

pub fn parse(bytes: &[u8]) -> parse::Result<Vec<ASTNode>> {
    let bytes = &mut &bytes[..];

    let mut nodes = Vec::new();
    loop {
        parse::whitespace_if_any(bytes);
        if bytes.len() == 0 {
            break;
        }
        let val = try!(parse::i32(bytes));
        let _ = try!(parse::literal(bytes, ";"));
        nodes.push(ASTNode::Value(Primitive::Number(val as f32)));
    }
    Ok(nodes)
}
