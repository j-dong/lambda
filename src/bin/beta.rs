extern crate lambda;

use lambda::expr;

use std::io;
use std::io::{Read, Write};
use std::fs;

fn main() {
    println!("beta reduction calculator");
    println!();
    println!("load FILE");
    println!("set EXPR");
    println!("beta [TIMES]");
    println!("print");
    println!("quit");
    println!();
    println!("% refers to the current expression");
    println!();
    let mut line = String::new();
    let mut working: Option<expr::lambda::LambdaExpr> = None;
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
        while line.ends_with(|c| c == '\r' || c == '\n') { line.pop().unwrap(); }
        if line.len() == 0 { continue; }
        if line == "quit" {
            return;
        } else if line == "print" {
            if let Some(ref e) = working {
                println!("{}", e);
                if let Err(err) = io::stdout().flush() {
                    eprintln!("error flushing stdout: {}", err);
                    return;
                }
            } else {
                eprintln!("no expression");
            }
        } else if line.starts_with("load ") {
            let filename = &line[5..];
            let mut f = match fs::File::open(filename) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("error opening '{}' for reading: {}", filename, err);
                    return
                },
            };
            let mut contents = String::new();
            if let Err(err) = f.read_to_string(&mut contents) {
                eprintln!("error reading file '{}': {}", filename, err);
                return;
            }
            match expr::lambda::parse(&contents) {
                Ok(e) => {
                    let mut next = e;
                    if let Some(old) = working {
                        next = next.replace("%", &old);
                    }
                    working = Some(next);
                },
                Err(err) => { eprintln!("syntax error in '{}': {}", filename, err); },
            }
        } else if line.starts_with("set ") {
            let expr_str = &line[4..];
            match expr::lambda::parse(expr_str) {
                Ok(e) => {
                    let mut next = e;
                    if let Some(old) = working {
                        next = next.replace("%", &old);
                    }
                    working = Some(next);
                },
                Err(err) => { eprintln!("syntax error: {}", err); },
            }
        } else if line.starts_with("beta") {
            let times = {
                if line.len() > 4 {
                    let times_str = &line[4..].trim();
                    match times_str.parse::<u32>() {
                        Ok(times) => times,
                        Err(err) => {
                            eprintln!("invalid number: {}", err);
                            continue
                        }
                    }
                } else { 1 }
            };
            if let Some(e) = working {
                let (next, app_times) = e.repeated_beta(times);
                eprintln!("reduced {} {}", app_times, if app_times == 1 { "time" } else { "times" });
                working = Some(next);
            } else {
                eprintln!("no expression");
            }
        } else {
            eprintln!("unrecognized command");
        }
    }
}
