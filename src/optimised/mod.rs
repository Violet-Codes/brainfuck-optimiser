use std::{fmt, collections::HashMap};
use std::rc::Rc;

pub mod interpreter;
pub mod repl;
pub mod optimiser;

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum ProcExpr {
    Lit(u8),
    Reg(i32),
    Add(Rc<ProcExpr>, Rc<ProcExpr>),
    Mul(Rc<ProcExpr>, Rc<ProcExpr>),
    Into(Rc<ProcExpr>, Rc<ProcExpr>) // Into(2, 8) = 4; Into(5, 4) = 52; Into(2, 3) = throw; Into(x, 0) = 0; Into(0, x) = throw; // throw when would forever-loop
}

#[derive(Debug)]
pub enum OptimisedBlock {
    Ask,
    Put,
    AtomicEffect(HashMap<i32, Rc<ProcExpr>>, i32),
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
                                lines.into_iter().map(|(register, expr)| format!("~#{} = {};", register, expr))
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

fn div_u8(x: u8, y: u8) -> Option<u8> {
    if x == 0 { return Some(0); }
    if y == 0 { return None; }

    let mut quant: u8 = 0;
    let mut acc: u8 = 0;
    // acc = x mod 1

    if acc % 2 != x % 2 { 'digit_0: {
        if y % 2 == 1 {
            acc = acc.wrapping_add(y << 0);
            quant += 1;
            break 'digit_0;
        }
        return None;
    }};
    // acc = x mod 2

    if acc % 4 != x % 4 { 'digit_1: {
        if y % 2 == 1 {
            acc = acc.wrapping_add(y << 1);
            quant += 2;
            break 'digit_1;
        }
        if y % 4 == 2 {
            acc = acc.wrapping_add(y << 0);
            quant += 1;
            break 'digit_1;
        }
        return None;
    }};
    // acc = x mod 4

    if acc % 8 != x % 8 { 'digit_2: {
        if y % 2 == 1 {
            acc = acc.wrapping_add(y << 2);
            quant += 4;
            break 'digit_2;
        }
        if y % 4 == 2 {
            acc = acc.wrapping_add(y << 1);
            quant += 2;
            break 'digit_2;
        }
        if y % 8 == 4 {
            acc = acc.wrapping_add(y << 0);
            quant += 1;
            break 'digit_2;
        }
        return None;
    }};
    // acc = x mod 8

    if acc % 16 != x % 16 { 'digit_3: {
        if y % 2 == 1 {
            acc = acc.wrapping_add(y << 3);
            quant += 8;
            break 'digit_3;
        }
        if y % 4 == 2 {
            acc = acc.wrapping_add(y << 2);
            quant += 4;
            break 'digit_3;
        }
        if y % 8 == 4 {
            acc = acc.wrapping_add(y << 1);
            quant += 2;
            break 'digit_3;
        }
        if y % 16 == 8 {
            acc = acc.wrapping_add(y << 0);
            quant += 1;
            break 'digit_3;
        }
        return None;
    }};
    // acc = x mod 16

    if acc % 32 != x % 32 { 'digit_4: {
        if y % 2 == 1 {
            acc = acc.wrapping_add(y << 4);
            quant += 16;
            break 'digit_4;
        }
        if y % 4 == 2 {
            acc = acc.wrapping_add(y << 3);
            quant += 8;
            break 'digit_4;
        }
        if y % 8 == 4 {
            acc = acc.wrapping_add(y << 2);
            quant += 4;
            break 'digit_4;
        }
        if y % 16 == 8 {
            acc = acc.wrapping_add(y << 1);
            quant += 2;
            break 'digit_4;
        }
        if y % 32 == 16 {
            acc = acc.wrapping_add(y << 0);
            quant += 1;
            break 'digit_4;
        }
        return None;
    }};
    // acc = x mod 32

    if acc % 64 != x % 64 { 'digit_5: {
        if y % 2 == 1 {
            acc = acc.wrapping_add(y << 5);
            quant += 32;
            break 'digit_5;
        }
        if y % 4 == 2 {
            acc = acc.wrapping_add(y << 4);
            quant += 16;
            break 'digit_5;
        }
        if y % 8 == 4 {
            acc = acc.wrapping_add(y << 3);
            quant += 8;
            break 'digit_5;
        }
        if y % 16 == 8 {
            acc = acc.wrapping_add(y << 2);
            quant += 4;
            break 'digit_5;
        }
        if y % 32 == 16 {
            acc = acc.wrapping_add(y << 1);
            quant += 2;
            break 'digit_5;
        }
        if y % 64 == 32 {
            acc = acc.wrapping_add(y << 0);
            quant += 1;
            break 'digit_5;
        }
        return None;
    }};
    // acc = x mod 64

    if acc % 128 != x % 128 { 'digit_6: {
        if y % 2 == 1 {
            acc = acc.wrapping_add(y << 6);
            quant += 64;
            break 'digit_6;
        }
        if y % 4 == 2 {
            acc = acc.wrapping_add(y << 5);
            quant += 32;
            break 'digit_6;
        }
        if y % 8 == 4 {
            acc = acc.wrapping_add(y << 4);
            quant += 16;
            break 'digit_6;
        }
        if y % 16 == 8 {
            acc = acc.wrapping_add(y << 3);
            quant += 8;
            break 'digit_6;
        }
        if y % 32 == 16 {
            acc = acc.wrapping_add(y << 2);
            quant += 4;
            break 'digit_6;
        }
        if y % 64 == 32 {
            acc = acc.wrapping_add(y << 1);
            quant += 2;
            break 'digit_6;
        }
        if y % 128 == 64 {
            acc = acc.wrapping_add(y << 0);
            quant += 1;
            break 'digit_6;
        }
        return None;
    }};
    // acc = x mod 128

    if acc != x { 'digit_7: {
        if y % 2 == 1 {
            // acc = acc.wrapping_add(y << 7);
            quant += 128;
            break 'digit_7;
        }
        if y % 4 == 2 {
            // acc = acc.wrapping_add(y << 6);
            quant += 64;
            break 'digit_7;
        }
        if y % 8 == 4 {
            // acc = acc.wrapping_add(y << 5);
            quant += 32;
            break 'digit_7;
        }
        if y % 16 == 8 {
            // acc = acc.wrapping_add(y << 4);
            quant += 16;
            break 'digit_7;
        }
        if y % 32 == 16 {
            // acc = acc.wrapping_add(y << 3);
            quant += 8;
            break 'digit_7;
        }
        if y % 64 == 32 {
            // acc = acc.wrapping_add(y << 2);
            quant += 4;
            break 'digit_7;
        }
        if y % 128 == 64 {
            // acc = acc.wrapping_add(y << 1);
            quant += 2;
            break 'digit_7;
        }
        if y == 128 {
            // acc = acc.wrapping_add(y << 0);
            quant += 1;
            break 'digit_7;
        }
        return None;
    }};
    // acc = x

    return Some(quant);
}