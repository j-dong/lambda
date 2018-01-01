mod lambda;
mod ski;
mod iota;
mod conv;
mod interpret;

use std::io;
use std::io::Write;

fn main() {
    println!("lambda calculus to iota converter");
    println!("type 'quit' to quit");
    let mut line = String::new();
    loop {
        print!("lc> ");
        if let Err(err) = io::stdout().flush() {
            eprintln!("error flushing stdout: {}", err);
            return;
        }
        line.clear();
        if let Err(err) = io::stdin().read_line(&mut line) {
            eprintln!("error reading from stdin: {}", err);
            return;
        }
        if line.len() == 0 {
            // end of file
            println!();
            return;
        }
        let line = line.trim();
        if line == "quit" {
            return;
        }
        let expr = match lambda::parse(line) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("syntax error: {}", err);
                continue
            },
        };
        let expr_str = format!("{}", expr);
        let iota = match ski::SKIExpr::from_lambda(expr).map(iota::IotaExpr::from) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("error converting to SKI: {}", err);
                continue
            }
        };
        println!("{} => {}", expr_str, iota.to_string());
    }
}
