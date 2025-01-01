trait Expr {

}
pub struct Binary impl Expr {
    left: Expr,
    operator: Token,
    right: Expr,
}

pub struct Grouping impl Expr {
    expression: Expr,
}

pub struct Literal impl Expr {
    value: Object,
}

pub struct Unary impl Expr {
    operator: Token,
    right: Expr,
}

