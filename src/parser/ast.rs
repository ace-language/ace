#[derive(Debug)]
pub struct Func {
    pub name: Ident,
    pub params: Vec<FuncParam>,
    pub ret_type: Option<Expr>,
    pub block: Block,
}

#[derive(Debug)]
pub struct FuncParam {
    pub name: Ident,
    pub acetype: Expr,
}

#[derive(Copy, Clone, Debug)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Index,
}
// Say, an identifier
pub type Ident = String;

#[derive(Debug)]
pub enum Expr {
    BinOp(Box<Expr>, Ops, Box<Expr>),
    Int(i64),
    Ident(Ident),
    Call(Box<Expr>, Vec<Expr>),
}

pub type Block = Vec<Stmt>;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Return(Option<Expr>),
    Assign(Expr, Expr),
    If(Expr, Block),
}

impl Expr {
    pub fn new_binop(left: Expr, op: Ops, right: Expr) -> Expr {
        Expr::BinOp(Box::new(left), op, Box::new(right))
    }
}
