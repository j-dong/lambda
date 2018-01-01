#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IotaExpr {
    Apply(Box<IotaExpr>, Box<IotaExpr>),
    Iota,
}
