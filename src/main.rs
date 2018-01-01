mod lambda;
mod ski;
mod iota;
mod conv;

fn main() {
    println!("{}", ski::SKIExpr::from_lambda(lambda::parse("\\f f (\\x \\y \\z x z (y z)) (\\x \\y x)").unwrap()).unwrap());
}
