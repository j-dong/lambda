use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum LambdaExpr {
    Variable(String),
    Apply(Box<LambdaExpr>, Box<LambdaExpr>),
    Lambda(String, Box<LambdaExpr>),
}

impl LambdaExpr {
    fn contains(&self, var: &str) -> bool {
        match self {
            &LambdaExpr::Variable(ref v) => v == var,
            &LambdaExpr::Apply(ref e1, ref e2) => e1.contains(var) || e2.contains(var),
            &LambdaExpr::Lambda(ref v, ref e) => v != var && e.contains(var),
        }
    }
}

struct Parenthesized<'a>(&'a LambdaExpr);

impl <'a> fmt::Display for Parenthesized<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let &Parenthesized(expr) = self;
        match expr {
            &LambdaExpr::Apply(_, _) => write!(f, "({})", expr),
            _ => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for LambdaExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &LambdaExpr::Variable(ref v) => write!(f, "{}", v),
            &LambdaExpr::Apply(ref e1, ref e2) => write!(f, "{} {}", e1, Parenthesized(&*e2)),
            &LambdaExpr::Lambda(ref v, ref e) => write!(f, "λ{} {}", v, e),
        }
    }
}

#[derive(Debug)]
struct Lexer<'a> {
    contents: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Ident,
    LParen,
    RParen,
    Lambda,
    EOF,
}

impl TokenType {
    fn can_begin_expr(&self) -> bool {
        match *self {
            TokenType::Ident => true,
            TokenType::LParen => true,
            TokenType::RParen => false,
            TokenType::Lambda => true,
            TokenType::EOF => false,
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(contents: &str) -> Lexer {
        Lexer { contents: contents.trim_left() }
    }

    fn peek(&self) -> TokenType {
        let ch = self.contents.chars().next();
        if let Some(ch) = ch {
            match ch {
                '(' => TokenType::LParen,
                ')' => TokenType::RParen,
                '\\' => TokenType::Lambda,
                'λ' => TokenType::Lambda,
                _ => TokenType::Ident,
            }
        } else {
            TokenType::EOF
        }
    }

    fn consume(&mut self) -> Option<&str> {
        match self.peek() {
            TokenType::EOF => None,
            TokenType::LParen | TokenType::RParen | TokenType::Lambda => {
                // find index of next code point
                let index = self.contents.char_indices().nth(1).map(|x| x.0).unwrap_or(self.contents.len());
                self.contents = &self.contents[index..].trim_left();
                None
            },
            TokenType::Ident => {
                // find index of first non-word character
                let index = self.contents.char_indices().find(|p| {
                    let c = p.1;
                    c == '(' || c == ')' || c == '\\' || c == 'λ' || c.is_whitespace()
                }).map(|x| x.0).unwrap_or(self.contents.len());
                let (ret, rest) = self.contents.split_at(index);
                self.contents = rest.trim_left();
                Some(ret)
            },
        }
    }
}

fn parse_atom(lex: &mut Lexer) -> Result<LambdaExpr, String> {
    match lex.peek() {
        TokenType::Ident => Ok(LambdaExpr::Variable(lex.consume().unwrap().to_string())),
        TokenType::LParen => {
            lex.consume();
            let result = parse_expr(lex)?;
            if lex.peek() != TokenType::RParen {
                Err(format!("expected RParen, got {:?}", lex.peek()))
            } else {
                lex.consume();
                Ok(result)
            }
        },
        TokenType::Lambda => {
            lex.consume();
            if lex.peek() != TokenType::Ident {
                return Err(format!("expected Ident after Lambda, got {:?}", lex.peek()));
            }
            let param = lex.consume().unwrap().to_string();
            Ok(LambdaExpr::Lambda(param, Box::new(parse_expr(lex)?)))
        },
        _ => Err(format!("expected expression, got {:?}", lex.peek()))
    }
}

fn parse_expr(lex: &mut Lexer) -> Result<LambdaExpr, String> {
    let mut tree = parse_atom(lex)?;
    while lex.peek().can_begin_expr() {
        tree = LambdaExpr::Apply(Box::new(tree), Box::new(parse_atom(lex)?));
    }
    Ok(tree)
}

pub fn parse(s: &str) -> Result<LambdaExpr, String> {
    let mut lex: Lexer = Lexer::new(s);
    parse_expr(&mut lex)
}

#[cfg(test)]
mod tests {
    use lambda::*;
    use lambda::LambdaExpr::*;

    fn x() -> String {
        "x".to_string()
    }

    fn y() -> String {
        "y".to_string()
    }

    #[test]
    fn single_variable() {
        assert_eq!(Ok(Variable(x())), parse("x"));
    }

    #[test]
    fn single_lambda() {
        assert_eq!(Ok(Lambda(x(), Box::new(Variable(x())))), parse("\\x x"));
    }

    #[test]
    fn single_parenthesis() {
        assert_eq!(Ok(Variable(x())), parse("(x)"));
    }

    #[test]
    fn single_application() {
        assert_eq!(Ok(Apply(Box::new(Variable(x())), Box::new(Variable(y())))), parse("x y"));
    }

    #[test]
    fn church_add() {
        assert_eq!(Ok(
                Lambda("m".to_string(), Box::new(
                        Lambda("n".to_string(), Box::new(
                                Lambda("f".to_string(), Box::new(
                                    Lambda("x".to_string(), Box::new(
                                            Apply(
                                                Box::new(Apply(
                                                        Box::new(Variable("m".to_string())),
                                                        Box::new(Variable("f".to_string())))),
                                                Box::new(Apply(
                                                        Box::new(Apply(
                                                                Box::new(Variable("n".to_string())),
                                                                Box::new(Variable("f".to_string())))),
                                                        Box::new(Variable("x".to_string())))))))))))))),
                parse("\\m \\n \\f \\x m f (n f x)"));
    }

    #[test]
    fn display_single_variable() {
        assert_eq!("x", format!("{}", parse("x").unwrap()));
    }

    #[test]
    fn display_single_lambda() {
        assert_eq!("λx x", format!("{}", parse("\\x x").unwrap()));
    }

    #[test]
    fn display_single_parenthesis() {
        assert_eq!("x", format!("{}", parse("(x)").unwrap()));
    }

    #[test]
    fn display_single_application() {
        assert_eq!("x y", format!("{}", parse("x y").unwrap()));
    }

    #[test]
    fn display_parenthesis() {
        assert_eq!("w x (y z)", format!("{}", parse("(w x) (y z)").unwrap()));
    }

    #[test]
    fn display_church_add() {
        assert_eq!("λm λn λf λx m f (n f x)", format!("{}", parse("\\m \\n \\f \\x m f (n f x)").unwrap()));
    }
}
