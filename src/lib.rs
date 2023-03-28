pub mod parser;
pub mod interpreter;
pub mod repl;

#[derive(Debug)]
pub enum BFRaw {
    Lft,
    Rgh,
    Inc,
    Dec,
    Ask,
    Put,
    Loop(Vec<BFRaw>)
}