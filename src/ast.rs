// Abstract Syntax Tree definitions for SSL

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    FunctionDecl(FunctionDecl),
    VariableDecl(VariableDecl),
    Assignment(Assignment),
    ExpressionStmt(Expression),
    If(IfStatement),
    For(ForLoop),
    While(WhileLoop),
    Return(Option<Expression>),
    Spawn(Vec<Statement>),
    TryRecover {
        try_block: Vec<Statement>,
        error_var: String,
        recover_block: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl {
    pub name: String,
    pub mutable: bool,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Vec<Statement>,
    pub else_block: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForLoop {
    pub var: String,
    pub iterable: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntLiteral(i64),
    StringLiteral(String),
    FunctionCall(FunctionCall),
    BinaryOp(BinaryOp),
    ListLiteral(Vec<Expression>),
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOp {
    pub left: Box<Expression>,
    pub op: Operator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    Lt,
    Le,
    Gt,
    Ge,
    Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    List(Box<Type>),
    Custom(String),
}
