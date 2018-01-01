#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IotaExpr {
    Apply(Box<IotaExpr>, Box<IotaExpr>),
    Iota,
}

impl IotaExpr {
    fn append_to_string(&self, s: &mut String) {
        match self {
            &IotaExpr::Iota => { s.push('i'); },
            &IotaExpr::Apply(ref e1, ref e2) => {
                s.push('*');
                e1.append_to_string(s);
                e2.append_to_string(s);
            },
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        self.append_to_string(&mut ret);
        ret
    }
}
