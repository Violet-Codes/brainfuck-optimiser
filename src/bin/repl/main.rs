use std::{collections::HashMap, sync::Mutex};
use std::io::Write;

use brainfuck_optimiser::optimised::{ *, interpreter::*, repl::* };

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
        display_help: display_help,
        display_optimisation: |bs: &Vec<OptimisedBlock>| println!("{bs:?}")
    };
    let memory = Mutex::new(HashMap::<i32, u8>::new());
    let mut ctx = BFCtx{
        index: 0,
        ask: || loop {
            match str::parse::<u8>(readln!("ask: ").as_str()) {
                Ok(x) => break x,
                Err(_) => println!("!NaN or not u8"),
            }
        },
        put: |x| println!("put: {x} ({})", x as char),
        get: |i| *memory.lock().unwrap().get(& i).unwrap_or(&0),
        set: |i, x| { memory.lock().unwrap().insert(i, x); },
        clear: || memory.lock().unwrap().clear()
    };
    println!("Welcome to the brainfuck REPL~ ❤️");
    println!("type ':h' for help");
    println!("");
    while rep(&mut interactor, &mut ctx) {}
}