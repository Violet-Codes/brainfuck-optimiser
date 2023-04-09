use std::fmt;

pub mod interpreter;
pub mod repl;
pub mod optimiser;

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
    Ask,
    Put,
    AtomicEffect(Vec<ProcAssign>, i32),
    Loop(Vec<OptimisedBlock>)
}

impl fmt::Display for ProcExpr {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcExpr::Lit(x) => write!(f, "{x}"),
            ProcExpr::Reg(r) => write!(f, "~#{r}"),
            ProcExpr::Add(a, b) => write!(f, "({a} + {b})"),
            ProcExpr::Mul(a, b) => write!(f, "{a} * {b}"),
            ProcExpr::Into(a, b) => write!(f, "({a} into {b})"),
        }
    }
}

impl fmt::Display for ProcAssign {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "~#{} = {};", self.register, self.expr)
    }
}

fn join_strings(mut ss: impl Iterator<Item=String>) -> String {
    if let Some(mut s) = ss.next() {
        ss.for_each(|s_| s = format!("{s}\n{s_}"));
        s
    } else {
        "".to_string()
    }
}

fn indent_string(s: String) -> String {
    format!("\t{}", (s).replace("\n", "\n\t"))
}

impl fmt::Display for OptimisedBlock {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            OptimisedBlock::Ask => write!(f, "ask"),
            OptimisedBlock::Put => write!(f, "put"),
            OptimisedBlock::AtomicEffect(lines, effect) => 
                if lines.is_empty() {
                    write!( f, "block {{}} (move {effect})")
                } else {
                    write!( f,
                        "block {{\n{}\n}} (move {effect})",
                        indent_string(
                            join_strings(
                                lines.into_iter().map(|line| format!("{line}"))
                            )
                        )
                    )
                },
            OptimisedBlock::Loop(lines) => write!( f,
                "loop [\n{}\n]",
                indent_string(
                    join_strings(
                        lines.into_iter().map(|line| format!("{line}"))
                    )
                )
            ),
        }
    }
}

pub fn byte_code_pretty(bs: & Vec<OptimisedBlock>) -> String {
    join_strings(bs.iter().map(|line| format!("{line}")))
}