use expr::lambda::LambdaExpr;

impl LambdaExpr {
    fn contains(&self, var: &str) -> bool {
        match self {
            &LambdaExpr::Variable(ref v) => v == var,
            &LambdaExpr::Apply(ref e1, ref e2) => e1.contains(var) || e2.contains(var),
            &LambdaExpr::Lambda(ref v, ref e) => v != var && e.contains(var),
        }
    }

    fn replace(self, param: &str, arg: &LambdaExpr) -> LambdaExpr {
        match self {
            LambdaExpr::Variable(v) => if v == param { arg.clone() } else { LambdaExpr::Variable(v) },
            LambdaExpr::Apply(e1, e2) => LambdaExpr::Apply(Box::new(e1.replace(param, arg)), Box::new(e2.replace(param, arg))),
            LambdaExpr::Lambda(v, e) => if v == param { LambdaExpr::Lambda(v, e) } else {
                if arg.contains(&v) {
                    // alpha-conversion
                    let new_name = {
                        let mut name = v.clone();
                        name.push('\'');
                        name
                    };
                    LambdaExpr::Lambda(new_name.clone(), Box::new(e.replace(&v, &LambdaExpr::Variable(new_name)).replace(param, arg)))
                } else {
                    LambdaExpr::Lambda(v, Box::new(e.replace(param, arg)))
                }
            }
        }
    }

    /// Performs beta-reduction on the first reducible term found.
    /// Second element of tuple is false if no reduction performed.
    pub fn beta(self) -> (LambdaExpr, bool) {
        match self {
            LambdaExpr::Variable(v) => (LambdaExpr::Variable(v), false),
            LambdaExpr::Lambda(v, e) => {
                let (e, res) = e.beta();
                (LambdaExpr::Lambda(v, Box::new(e)), res)
            },
            LambdaExpr::Apply(e1, e2) => {
                let (e1,) = (*e1,);
                match e1 {
                    LambdaExpr::Variable(v) => {
                        let (e2, res) = e2.beta();
                        (LambdaExpr::Apply(Box::new(LambdaExpr::Variable(v)), Box::new(e2)), res)
                    },
                    e1 @ LambdaExpr::Apply(_, _) => {
                        let (e1, res) = e1.beta();
                        let (e2, res) = if res { (*e2, res) } else { e2.beta() };
                        (LambdaExpr::Apply(Box::new(e1), Box::new(e2)), res)
                    },
                    LambdaExpr::Lambda(v, e) => (e.replace(&v, &e2), true),
                }
            },
        }
    }

    /// Performs beta reduction up to `limit` times.
    /// Returns the number of times reduced.
    /// Will only be `limit` if no normal form found.
    pub fn repeated_beta(self, limit: u32) -> (LambdaExpr, u32) {
        let mut ret = self;
        for i in 0..limit {
            let (next, res) = ret.beta();
            if !res {
                return (next, i)
            }
            ret = next;
        }
        (ret, limit)
    }
}

#[cfg(test)]
mod tests {
    use expr::lambda::*;

    #[test]
    fn replace_alpha() {
        assert_eq!(parse("\\y' y").unwrap(), parse("\\y x").unwrap().replace("x", &LambdaExpr::Variable("y".to_string())));
    }

    #[test]
    fn beta_id() {
        assert_eq!((parse("x").unwrap(), true), parse("(\\x x) x").unwrap().beta());
    }

    #[test]
    fn beta_eta() {
        // eta-conversion
        assert_eq!((parse("y").unwrap(), true), parse("(\\x y) x").unwrap().beta());
    }

    #[test]
    fn beta_const() {
        assert_eq!((parse("y").unwrap(), 2), parse("(\\x \\y x) y x").unwrap().repeated_beta(3));
    }

    #[test]
    fn beta_succ() {
        assert_eq!((parse("\\f \\x f (f x)").unwrap(), 3), parse("(\\n \\f \\x f (n f x)) (\\f \\x f x)").unwrap().repeated_beta(10));
    }
}
