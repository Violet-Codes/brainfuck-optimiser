pub mod interpreter;
pub mod repl;
pub mod optimiser;
pub mod attributes;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum ProcExpr {
    Lit(u8),
    Reg(i32),
    Add(Box<ProcExpr>, Box<ProcExpr>),
    Mul(Box<ProcExpr>, Box<ProcExpr>),
    Into(Box<ProcExpr>, Box<ProcExpr>) // Into(2, 8) = 4; Into(5, 4) = 52; Into(2, 3) = throw; Into(x, 0) = 0; Into(0, x) = throw; // throw when would forever-loop
}

#[derive(Debug)]
pub struct ProcAssign {
    register: i32,
    expr: ProcExpr
}

#[derive(Debug)]
pub enum OptimisedBlock {
    AtomicEffect(Vec<ProcAssign>, i32),
    Loop(Vec<OptimisedBlock>)
}