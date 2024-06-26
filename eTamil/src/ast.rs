pub enum ASTNode {
    Program(Vec<ASTNode>),
    Assignment(String, Box<ASTNode>),
    Expression(Box<ASTNode>, Vec<(String, Box<ASTNode>)>),
    Term(Box<ASTNode>, Vec<(String, Box<ASTNode>)>),
    Factor(String),
    Number(i64),
}
