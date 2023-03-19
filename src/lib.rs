pub mod parser;
pub mod interpreter;

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