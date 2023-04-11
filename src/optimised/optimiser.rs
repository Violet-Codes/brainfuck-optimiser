use std::collections::{HashMap, HashSet, BTreeMap};
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
            ProcExpr::Lit(v) => ProcExpr::Lit(*v),
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
        match y.as_ref() {
            ProcExpr::Reg(r) => if *r == s { x } else { Rc::new(ProcExpr::Reg(*r)) },
            ProcExpr::Lit(v) => Rc::new(ProcExpr::Lit(*v)),
            ProcExpr::Add(a, b) => Rc::new(ProcExpr::Add(
                replace(s, x.clone(), a.clone()),
                replace(s, x.clone(), b.clone())
            )),
            ProcExpr::Mul(a, b) => Rc::new(ProcExpr::Mul(
                replace(s, x.clone(), a.clone()),
                replace(s, x.clone(), b.clone())
            )),
            ProcExpr::Into(a, b) => Rc::new(ProcExpr::Into(
                replace(s, x.clone(), a.clone()),
                replace(s, x.clone(), b.clone())
            ))
        }
    }

    let OptimisedBlock::AtomicEffect(xs, i) = a else { return None; };
    let OptimisedBlock::AtomicEffect(ys, j) = b else { return None; };
    let mut new_xs: HashMap<i32, Rc<ProcExpr>> = xs.clone();
    let mut new_ys: HashMap<i32, Rc<ProcExpr>> = ys.iter().map(|(register, expr)| (register + i, shift(*i, expr.clone()))).collect();
    for (key_x, expr_x) in new_xs.iter() {
        for (_, expr_y) in new_ys.iter_mut() {
            *expr_y = replace(*key_x, expr_x.clone(), expr_y.clone());
        }
    }
    new_xs.extend(new_ys.into_iter());
    Some(OptimisedBlock::AtomicEffect(new_xs, i + j))
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Multinomial{
    coefficients: HashMap<BTreeMap<Rc<ProcExpr>, u32>, u8>,
    symbols: HashSet<Rc<ProcExpr>>
}

impl Multinomial {
    pub fn symbol(expr: Rc<ProcExpr>) -> Multinomial {
        let mut res = Multinomial::default();
        let mut map: BTreeMap<Rc<ProcExpr>, u32> = BTreeMap::new();
        map.insert(expr.clone(), 1);
        res.coefficients.insert(map, 1);
        res
    }

    pub fn value(x: u8) -> Multinomial {
        let mut res = Multinomial::default();
        if x == 0 { return res; }
        res.coefficients.insert(BTreeMap::new(), x);
        res
    }

    fn _reduce(self) -> Multinomial {
        let mut res = Multinomial::default();
        res.coefficients = self.coefficients.into_iter().filter(|(_, x)| x != &0).collect();
        res
    }

    fn _gen_symbols(&mut self) -> () {
        for (term, _) in self.coefficients.iter() {
            for (symbol, _) in term {
                self.symbols.insert(symbol.clone());
            }
        }
    }

    pub fn add(&self, other: &Self) -> Multinomial {
        let mut res = Multinomial::default();
        let self_terms = self.coefficients.keys().collect::<HashSet<&BTreeMap<Rc<ProcExpr>, u32>>>();
        let other_terms = other.coefficients.keys().collect::<HashSet<&BTreeMap<Rc<ProcExpr>, u32>>>();
        for term in self_terms.union(&other_terms).map(|map| (*map).clone()) {
            let coefficient = self.coefficients.get(&term).unwrap_or(&0).wrapping_add(*other.coefficients.get(&term).unwrap_or(&0));
            res.coefficients.insert(
                term,
                coefficient
            );
        }
        res = res._reduce();
        res._gen_symbols();
        res
    }

    pub fn mul(&self, other: &Self) -> Multinomial {
        let mut res = Multinomial::default();
        for (self_term, self_coefficient) in self.coefficients.iter() {
            for (other_term, other_coefficient) in other.coefficients.iter() {
                let mut term = BTreeMap::new();
                for (symbol, power) in self_term {
                    term.insert(symbol.clone(), term.get(symbol).unwrap_or(&0) + power);
                }
                for (symbol, power) in other_term {
                    term.insert(symbol.clone(), term.get(symbol).unwrap_or(&0) + power);
                }
                let coefficient = res.coefficients.get(&term).unwrap_or(&0).wrapping_add(self_coefficient.wrapping_mul(*other_coefficient));
                res.coefficients.insert(term, coefficient);
            }
        }
        res = res._reduce();
        res._gen_symbols();
        res
    }

    pub fn as_val(&self) -> Rc<ProcExpr> {
        let mut coefficients = self.coefficients.iter();
        if let Some((term, coefficient)) = coefficients.next() {
            fn as_prod(term: &BTreeMap<Rc<ProcExpr>, u32>, coefficient: &u8) -> Rc<ProcExpr> {
                let mut expr = Rc::new(ProcExpr::Lit(*coefficient));
                for (symbol, power) in term {
                    for _ in 0..*power {
                        expr = Rc::new(ProcExpr::Mul(expr, symbol.clone()))
                    }
                }
                if *coefficient == 1 {
                    if let ProcExpr::Mul(_, e) = expr.as_ref() {
                        return e.clone();
                    }
                }
                expr
            }
            let mut expr = as_prod(term, coefficient);
            for (term, coefficient) in coefficients {
                expr = Rc::new(ProcExpr::Add(expr, as_prod(term, coefficient)))
            }
            expr
        } else {
            Rc::new(ProcExpr::Lit(0))
        }
    }
}

fn reduce(
    expr: Rc<ProcExpr>
)
    -> Rc<ProcExpr>
{
    fn reduce_to_multinomial(
        expr: Rc<ProcExpr>
    )
        -> Multinomial
    {
        match expr.as_ref() {
            ProcExpr::Lit(x) => Multinomial::value(*x),
            ProcExpr::Reg(_) => Multinomial::symbol(expr),
            ProcExpr::Add(a, b) => reduce_to_multinomial(a.clone()).add(& reduce_to_multinomial(b.clone())),
            ProcExpr::Mul(a, b) => reduce_to_multinomial(a.clone()).mul(& reduce_to_multinomial(b.clone())),
            ProcExpr::Into(a, b) => {
                let expr_a = reduce(a.clone());
                let expr_b = reduce(b.clone());
                if expr_b.as_ref() == &ProcExpr::Lit(0) {
                    return Multinomial::default();
                }
                if expr_a.as_ref() == &ProcExpr::Lit(1) {
                    return reduce_to_multinomial(expr_b);
                }
                let ProcExpr::Lit(a_) = expr_a.as_ref() else {
                    return Multinomial::symbol(Rc::new(ProcExpr::Into(expr_a, expr_b)));
                };
                let ProcExpr::Lit(b_) = expr_b.as_ref() else {
                    return Multinomial::symbol(Rc::new(ProcExpr::Into(expr_a, expr_b)));
                };
                let Some(c) = div_u8(*b_, *a_) else {
                    return Multinomial::symbol(Rc::new(ProcExpr::Into(expr_a, expr_b)));
                };
                Multinomial::symbol(Rc::new(ProcExpr::Lit(c)))
            },
        }
    }
    reduce_to_multinomial(expr).as_val()
}

fn try_loop_optimise(
    b: & OptimisedBlock
)
    -> Option<OptimisedBlock>
{
    fn registers(
        expr: Rc<ProcExpr>
    )
        -> HashSet<i32>
    {
        match expr.as_ref() {
            ProcExpr::Lit(_) => HashSet::new(),
            ProcExpr::Reg(r) => {
                let mut set = HashSet::new();
                set.insert(*r);
                set
            },
            ProcExpr::Add(a, b) => registers(a.clone()).union(&registers(b.clone())).map(|r| r.clone()).collect(),
            ProcExpr::Mul(a, b) => registers(a.clone()).union(&registers(b.clone())).map(|r| r.clone()).collect(),
            ProcExpr::Into(a, b) => registers(a.clone()).union(&registers(b.clone())).map(|r| r.clone()).collect(),
        }
    }

    match b {
        OptimisedBlock::AtomicEffect(lines, 0) => {
            let Some(index_expr) = lines.get(& 0) else { return None; };
            let subtraction = reduce(
                Rc::new(ProcExpr::Add(
                    Rc::new(ProcExpr::Reg(0)),
                    Rc::new(ProcExpr::Mul(
                        Rc::new(ProcExpr::Lit(255)),
                        index_expr.clone()
                    ))
                ))
            );

            let mut new_lines = HashMap::<i32, Rc<ProcExpr>>::new();

            let cycles = Rc::new(ProcExpr::Into(
                subtraction.clone(),
                Rc::new(ProcExpr::Reg(0))
            ));
            
            let rs = registers(subtraction);
            for register in lines.keys() {
                if rs.contains(register) { return None; }
            }

            new_lines.insert(0, Rc::new(ProcExpr::Lit(0)));

            for (r, expr) in lines {
                let addition = reduce(
                    Rc::new(ProcExpr::Add(
                        expr.clone(),
                        Rc::new(ProcExpr::Mul(
                            Rc::new(ProcExpr::Lit(255)),
                            Rc::new(ProcExpr::Reg(*r))
                        ))
                    ))
                );
                if *r == 0 { continue }
                let rs = registers(addition.clone());
                for register in lines.keys() {
                    if rs.contains(register) { return None; }
                }

                new_lines.insert(*r, Rc::new(ProcExpr::Add(
                    Rc::new(ProcExpr::Reg(*r)),
                    Rc::new(ProcExpr::Mul(
                        cycles.clone(),
                        addition
                    ))
                )));
            }
            Some(OptimisedBlock::AtomicEffect(new_lines, 0))
        }
        _ => None
    }
}

fn merge_all(
    bs: Vec<OptimisedBlock>
)
    -> Vec<OptimisedBlock>
{
    let mut iter_bs = bs.into_iter();
    let Some(a) = iter_bs.next() else { return vec![]; };
    let Some(b) = iter_bs.next() else { return vec![a]; };
    let Some(c) = try_merge(&a, &b) else {
        let mut from_a = vec![a];
        let mut from_b = vec![b];
        from_b.extend(iter_bs);
        from_a.extend(merge_all(from_b).into_iter());
        return from_a;
    };
    let mut from_c = vec![c];
    from_c.extend(iter_bs);
    merge_all(from_c)
}

fn reduce_all(
    bs: Vec<OptimisedBlock>
)
    ->  Vec<OptimisedBlock>
{
    bs.into_iter().map(|b| match b {
        OptimisedBlock::AtomicEffect(lines, i) => OptimisedBlock::AtomicEffect(
            lines.into_iter().map(|(r, expr)| (r, reduce(expr))).collect(),
            i
        ),
        _ => b
    }).collect()
}

fn optimise_loops(
    bs: Vec<OptimisedBlock>
)
    ->  Vec<OptimisedBlock>
{
    bs.into_iter().map(|b| match b {
        OptimisedBlock::Loop(bs_) => {
            let optimised_bs_ = optimise(bs_);
            if optimised_bs_.len() == 1 {
                try_loop_optimise(optimised_bs_.first().unwrap()).unwrap_or(OptimisedBlock::Loop(optimised_bs_))
            } else {
                OptimisedBlock::Loop(optimised_bs_)
            }
        },
        _ => b
    }).collect()
}

fn optimise(
    mut bs: Vec<OptimisedBlock>
)
    ->  Vec<OptimisedBlock>
{
    bs = optimise_loops(bs);
    bs = merge_all(bs);
    bs = reduce_all(bs);
    bs
}

pub fn optimising_convert(
    raw: Vec<BFRaw>
)
    -> Vec<OptimisedBlock>
{
    optimise(convert(raw))
}