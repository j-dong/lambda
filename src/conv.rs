use lambda::LambdaExpr;
use ski::SKIExpr;

use std::fmt;

impl From<SKIExpr> for LambdaExpr {
    fn from(expr: SKIExpr) -> LambdaExpr {
        match expr {
            SKIExpr::Apply(e1, e2) =>
                LambdaExpr::Apply(Box::new(LambdaExpr::from(*e1)), Box::new(LambdaExpr::from(*e2))),
            SKIExpr::S =>
                LambdaExpr::Lambda("x".to_string(),
                    Box::new(LambdaExpr::Lambda("y".to_string(),
                        Box::new(LambdaExpr::Lambda("z".to_string(),
                            Box::new(LambdaExpr::Apply(
                                Box::new(LambdaExpr::Apply(
                                    Box::new(LambdaExpr::Variable("x".to_string())),
                                    Box::new(LambdaExpr::Variable("y".to_string())))),
                                Box::new(LambdaExpr::Apply(
                                    Box::new(LambdaExpr::Variable("y".to_string())),
                                    Box::new(LambdaExpr::Variable("z".to_string()))))))))))),
            SKIExpr::K =>
                LambdaExpr::Lambda("x".to_string(),
                    Box::new(LambdaExpr::Lambda("y".to_string(),
                        Box::new(LambdaExpr::Variable("x".to_string()))))),
            SKIExpr::I =>
                LambdaExpr::Lambda("x".to_string(),
                    Box::new(LambdaExpr::Variable("x".to_string()))),
        }
    }
}

#[derive(Debug)]
enum IntExpr {
    Variable(String),
    Apply(Box<IntExpr>, Box<IntExpr>),
    Lambda(String, Box<IntExpr>),
    S, K, I,
}


impl From<LambdaExpr> for IntExpr {
    fn from(expr: LambdaExpr) -> IntExpr {
        match expr {
            LambdaExpr::Variable(s) => IntExpr::Variable(s),
            LambdaExpr::Apply(e1, e2) => IntExpr::Apply(Box::new(IntExpr::from(*e1)), Box::new(IntExpr::from(*e2))),
            LambdaExpr::Lambda(v, e) => IntExpr::Lambda(v, Box::new(IntExpr::from(*e))),
        }
    }
}

impl From<SKIExpr> for IntExpr {
    fn from(expr: SKIExpr) -> IntExpr {
        match expr {
            SKIExpr::Apply(e1, e2) => IntExpr::Apply(Box::new(IntExpr::from(*e1)), Box::new(IntExpr::from(*e2))),
            SKIExpr::S => IntExpr::S,
            SKIExpr::K => IntExpr::K,
            SKIExpr::I => IntExpr::I,
        }
    }
}

impl IntExpr {
    fn contains(&self, var: &str) -> bool {
        match self {
            &IntExpr::Variable(ref v) => v == var,
            &IntExpr::Apply(ref e1, ref e2) => e1.contains(var) || e2.contains(var),
            &IntExpr::Lambda(ref v, ref e) => v != var && e.contains(var),
            _ => false
        }
    }

    fn is_var(&self, var: &str) -> bool {
        if let &IntExpr::Variable(ref v) = self {
            v == var
        } else { false }
    }

    fn translate(self) -> IntExpr {
        match self {
            IntExpr::Apply(e1, e2) => IntExpr::Apply(Box::new(IntExpr::translate(*e1)), Box::new(IntExpr::translate(*e2))),
            IntExpr::Lambda(v, e) => if !e.contains(&v) {
                    IntExpr::Apply(Box::new(IntExpr::K), Box::new(IntExpr::translate(*e)))
                } else {
                    let (e,) = (*e,); // see #16223
                    match e {
                        IntExpr::Variable(v2) => {
                                assert_eq!(v, v2); // otherwise e.contains(v) must be false
                                IntExpr::I
                            },
                        IntExpr::Apply(e1, e2) =>
                            if e2.is_var(&v) && !e1.contains(&v) {
                                e1.translate()
                            } else {
                                IntExpr::Apply(
                                    Box::new(IntExpr::Apply(
                                        Box::new(IntExpr::S),
                                        Box::new(IntExpr::translate(
                                            IntExpr::Lambda(v.clone(), e1))))),
                                    Box::new(IntExpr::translate(
                                            IntExpr::Lambda(v, e2))))
                            },
                        e @ IntExpr::Lambda(_, _) =>
                                IntExpr::translate(
                                    IntExpr::Lambda(v,
                                        Box::new(IntExpr::from(IntExpr::translate(e))))),
                        _ => unreachable!() // (S, K, I).contains(_) == false
                    }
                },
            e => e
        }
    }
}

impl SKIExpr {
    pub fn from_lambda(expr: LambdaExpr) -> Result<SKIExpr, String> {
        SKIExpr::from_int(IntExpr::from(expr).translate())
    }

    fn from_int(expr: IntExpr) -> Result<SKIExpr, String> {
        match expr {
            IntExpr::Apply(e1, e2) => Ok(SKIExpr::Apply(Box::new(SKIExpr::from_int(*e1)?), Box::new(SKIExpr::from_int(*e2)?))),
            IntExpr::S => Ok(SKIExpr::S),
            IntExpr::K => Ok(SKIExpr::K),
            IntExpr::I => Ok(SKIExpr::I),
            IntExpr::Variable(v) => Err(format!("free variable: {}", v)),
            l @ IntExpr::Lambda(_, _) => Err(format!("untranslated lambda: {:?}", l)),
        }
    }

    fn display_lambda(&self) -> LambdaExpr {
        match self {
            &SKIExpr::Apply(ref e1, ref e2) => LambdaExpr::Apply(Box::new(e1.display_lambda()), Box::new(e2.display_lambda())),
            &SKIExpr::S => LambdaExpr::Variable("S".to_string()),
            &SKIExpr::K => LambdaExpr::Variable("K".to_string()),
            &SKIExpr::I => LambdaExpr::Variable("I".to_string()),
        }
    }
}

impl fmt::Display for SKIExpr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.display_lambda())
    }
}

#[cfg(test)]
mod tests {
    use conv::*;
    use lambda;

    #[test]
    fn id_to_ski() {
        assert_eq!("I", format!("{}", SKIExpr::from_lambda(lambda::parse("\\x x").unwrap()).unwrap()));
    }

    #[test]
    fn const_to_ski() {
        assert_eq!("K", format!("{}", SKIExpr::from_lambda(lambda::parse("\\x \\y x").unwrap()).unwrap()));
    }

    #[test]
    fn flip_to_ski() {
        assert_eq!("S (K (S I)) K", format!("{}", SKIExpr::from_lambda(lambda::parse("\\x \\y y x").unwrap()).unwrap()));
    }
}
