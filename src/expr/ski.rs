#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SKIExpr {
    Apply(Box<SKIExpr>, Box<SKIExpr>),
    S,
    K,
    I,
}
