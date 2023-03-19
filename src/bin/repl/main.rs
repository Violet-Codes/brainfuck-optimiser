use std::collections::HashMap;
use std::io::Write;
use nibbler::errors::show_error;

use brainfuck_optimiser::{ parser::*, interpreter::* };

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
    };
    let show_info = |info: TextInfo| format!("at {}:{}", info.line, info.index);
    let mut ctx = BFCtx{
        index: 0,
        memory: HashMap::new(),
        ask: || loop {
            match str::parse::<u8>(readln!("ask: ").as_str()) {
                Ok(x) => break x,
                Err(_) => println!("!NaN or not u8"),
            }
        },
        put: |x| println!("put: {x}")
    };
    println!("Welcome to the brainfuck REPL~ ❤️");
    println!("type ':h' for help");
    println!("");
    'repl: loop {
        let s = readln!("$ ");
        let mut iter = TextIter{
            iter: s.chars(),
            line: 0,
            index: 0
        };
        if s.chars().nth(0) == Some(':') {
            match parse_bfcmd()(&mut iter) {
                Ok(BFCMD::Exit) => break 'repl,
                Ok(BFCMD::Read(x)) => println!("#{x}: {}", ctx.memory.get(& x).unwrap_or(& 0)),
                Ok(BFCMD::Clear) => ctx.memory.clear(),
                Ok(BFCMD::Help) => display_help(),
                Ok(BFCMD::Find) => println!("head: #{}", ctx.index),
                Ok(BFCMD::Move(x)) => ctx.index = x,
                Err(err) => eprintln!("{}\n...whilst parsing instruction", show_error("".to_string(), & show_info, err)),
            };
        } else {
            match parse_program()(&mut iter) {
                Ok(is) => run_bfraw(&mut ctx, & is),
                Err(err) => eprintln!("{}\n...whilst parsing input", show_error("".to_string(), & show_info, err)),
            };
        }
    }
}