use std::collections::HashMap;
use std::io::Write;

use brainfuck_optimiser::{ interpreter::*, repl::* };

macro_rules! readln {
    ($s:expr) => {{
        print!($s);
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        buffer
    }};
    () => {{
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        buffer
    }};
}

pub fn main() {
    fn display_help() {
        println!("");
        println!(":q           \t\tquits");
        println!(":r <register>\t\tdisplays the contents of given register");
        println!(":c           \t\tclears all registers");
        println!(":h           \t\tdisplays this help text");
        println!(":f           \t\tfinds the register of the head");
        println!(":m <register>\t\tmoves the head to a specific register");
        println!("<input>      \t\truns the brainfuck code (':' is not a valid comment char)");
        println!("");
    }
    let mut interactor = ConsoleInteractor{
        readln: |s| readln!("{s}"),
        writeln: |s| println!("{s}"),
        write_errln: |s| eprintln!("{s}"),
        display_help: display_help
    };
    let mut ctx = BFCtx{
        index: 0,
        memory: HashMap::new(),
        ask: || loop {
            match str::parse::<u8>(readln!("ask: ").as_str()) {
                Ok(x) => break x,
                Err(_) => println!("!NaN or not u8"),
            }
        },
        put: |x| println!("put: {x} ({})", x as char)
    };
    println!("Welcome to the brainfuck REPL~ ❤️");
    println!("type ':h' for help");
    println!("");
    while REP(&mut interactor, &mut ctx) {}
}