#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Ident(String),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
    Call(String, Vec<Expr>),
    Signal(Box<Expr>, Box<Expr>), // target, data
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    UniverseDecl {
        name: String,
        energy: Option<f64>,
        body: Vec<Stmt>,
    },
    FuncDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    IfStmt {
        cond: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    WhileStmt {
        cond: Expr,
        body: Vec<Stmt>,
    },
    AssignStmt(String, Expr),
    ReturnStmt(Expr),
    ExprStmt(Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
