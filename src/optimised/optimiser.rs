use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::*;
use super::*;

pub fn convert(
    raw: Vec<BFRaw>
)
    -> Vec<OptimisedBlock>
{
    let mut bs: Vec<OptimisedBlock> = vec![];
    let mut diff: HashMap<i32, u8> = HashMap::new();
    let mut offset: i32 = 0;

    macro_rules! flush_block {
        () => {
            if '_if : {
                for v in diff.values() {
                    if *v != 0 { break '_if true; }
                }
                if offset != 0 { break '_if true; }
                false
            } {
                bs.push(
                    OptimisedBlock::AtomicEffect(
                        diff.into_iter().map(|(k, v)| (
                            k,
                            Rc::new(ProcExpr::Add(
                                Rc::new(ProcExpr::Reg(k)),
                                Rc::new(ProcExpr::Lit(v))
                            ))
                        )).collect(),
                        offset
                    )
                );
            }
        };
    }
    macro_rules! flush_block_reset {
        () => {
            if '_if : {
                for v in diff.values() {
                    if *v != 0 { break '_if true; }
                }
                if offset != 0 { break '_if true; }
                false
            } {
                bs.push(
                    OptimisedBlock::AtomicEffect(
                        diff.into_iter().map(|(k, v)| (
                            k,
                            Rc::new(ProcExpr::Add(
                                Rc::new(ProcExpr::Reg(k)),
                                Rc::new(ProcExpr::Lit(v))
                            ))
                        )).collect(),
                        offset
                    )
                );
                diff = HashMap::new();
                offset = 0;
            }
        };
    }

    for i in raw {
        match i {
            BFRaw::Lft => offset -= 1,
            BFRaw::Rgh => offset += 1,
            BFRaw::Inc => { diff.insert(offset, diff.get(& offset).unwrap_or(& 0).wrapping_add(1)); },
            BFRaw::Dec => { diff.insert(offset, diff.get(& offset).unwrap_or(& 0).wrapping_add(255)); },
            BFRaw::Ask => {
                flush_block_reset!();
                bs.push(OptimisedBlock::Ask)
            },
            BFRaw::Put => {
                flush_block_reset!();
                bs.push(OptimisedBlock::Put)
            },
            BFRaw::Loop(is) => {
                flush_block_reset!();
                bs.push(OptimisedBlock::Loop(convert(is)))
            },
        }
    };
    flush_block!();
    bs
}

fn try_merge(
    a: & OptimisedBlock,
    b: & OptimisedBlock
)
    -> Option<OptimisedBlock>
{
    fn shift(
        s: i32,
        x: Rc<ProcExpr>
    )
        -> Rc<ProcExpr>
    {
        Rc::new(match x.as_ref() {
            ProcExpr::Reg(r) => ProcExpr::Reg(r + s),
            ProcExpr::Lit(v) => ProcExpr::Lit(v.clone()),
            ProcExpr::Add(a, b) => ProcExpr::Add(
                shift(s, a.clone()),
                shift(s, b.clone())
            ),
            ProcExpr::Mul(a, b) => ProcExpr::Mul(
                shift(s, a.clone()),
                shift(s, b.clone())
            ),
            ProcExpr::Into(a, b) => ProcExpr::Into(
                shift(s, a.clone()),
                shift(s, b.clone())
            )
        })
    }
    pub fn replace(
        s: i32,
        x: Rc<ProcExpr>,
        y: Rc<ProcExpr>
    )
        -> Rc<ProcExpr>
    {
        Rc::new(match y.as_ref() {
            ProcExpr::Reg(r) => ProcExpr::Reg(r + s),
            ProcExpr::Lit(v) => ProcExpr::Lit(v.clone()),
            ProcExpr::Add(a, b) => ProcExpr::Add(
                replace(s, x.clone(), a.clone()),
                replace(s, x.clone(), b.clone())
            ),
            ProcExpr::Mul(a, b) => ProcExpr::Mul(
                replace(s, x.clone(), a.clone()),
                replace(s, x.clone(), b.clone())
            ),
            ProcExpr::Into(a, b) => ProcExpr::Into(
                replace(s, x.clone(), a.clone()),
                replace(s, x.clone(), b.clone())
            )
        })
    }

    if let OptimisedBlock::AtomicEffect(xs, i) = a {
        if let OptimisedBlock::AtomicEffect(ys, j) = b {
            let mut new_xs: HashMap<i32, Rc<ProcExpr>> = xs.clone();
            let mut new_ys: HashMap<i32, Rc<ProcExpr>> = ys.iter().map(|(register, expr)| (register - i, shift(-i, expr.clone()))).collect();
            for (key_x, expr_x) in new_xs.iter() {
                for (_, expr_y) in new_ys.iter_mut() {
                    *expr_y = replace(*key_x, expr_x.clone(), expr_y.clone());
                }
            }
            new_xs.extend(new_ys.into_iter());
            Some(OptimisedBlock::AtomicEffect(new_xs, i + j))
        } else {
            None
        }
    } else {
        None
    }
}

enum IterChain<T> {
    Single(std::vec::IntoIter<T>),
    Chain(Box<IterChain<T>>, Box<IterChain<T>>),
}

impl<T> Iterator for IterChain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterChain::Single(ref mut iter) => iter.next(),
            IterChain::Chain(ref mut lft, ref mut rgh) => {
                if let Some(val) = lft.next() {
                    Some(val)
                } else {
                    unsafe {
                        *self = std::ptr::read(rgh.as_mut());
                    }
                    self.next()
                }
            }
        }
    }
}

fn get_registers(
    expr: Rc<ProcExpr>
)
    -> HashSet<i32>
{
    fn get_registers_inner(
        expr: Rc<ProcExpr>,
        set: &mut HashSet<i32>
    )
        -> ()
    {
        match expr.as_ref() {
            ProcExpr::Reg(r) => { set.insert(*r); },
            ProcExpr::Add(a, b) => {
                get_registers_inner(a.clone(), set);
                get_registers_inner(b.clone(), set);
            },
            ProcExpr::Mul(a, b) => {
                get_registers_inner(a.clone(), set);
                get_registers_inner(b.clone(), set);
            },
            ProcExpr::Into(a, b) => {
                get_registers_inner(a.clone(), set);
                get_registers_inner(b.clone(), set);
            },
            ProcExpr::Lit(_) => ()
        }
    }
    let mut set = HashSet::new();
    get_registers_inner(expr, &mut set);
    return set
}

fn try_loop_optimise(
    b: & OptimisedBlock
)
    -> Option<OptimisedBlock>
{
    fn is_independent(
        r: i32,
        expr: Rc<ProcExpr>
    )
        -> bool
    {
        match expr.as_ref() {
            ProcExpr::Lit(_) => true,
            ProcExpr::Reg(s) => r != *s,
            ProcExpr::Add(a, b) => is_independent(r, a.clone()) && is_independent(r, b.clone()),
            ProcExpr::Mul(a, b) => is_independent(r, a.clone()) && is_independent(r, b.clone()),
            ProcExpr::Into(a, b) => is_independent(r, a.clone()) && is_independent(r, b.clone()),
        }
    }

    match b {
        OptimisedBlock::AtomicEffect(lines, 0) => {
            let Some(index_expr) = lines.get(& 0) else { return None; };
            //check if effect on ~#0 has a precalculatable number of itterations
            for register in lines.keys() {
                if *register != 0 && !is_independent(*register, index_expr.clone()) { return None; }
            }
            todo!()
        }
        _ => None
    }
}

pub fn optimising_convert(
    raw: Vec<BFRaw>
)
    -> Vec<OptimisedBlock>
{
    convert(raw)
}