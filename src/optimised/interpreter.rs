use std::{collections::HashMap, future::Future, pin::Pin};

use super::*;

pub use crate::interpreter::{BFCtx, AsyncBFCtx};

#[cfg(test)]
mod tests {

    #[test]
    fn simple_test_div_u8() {
        use super::*;

        for x_ in 1..256 {
            let x: u8 = x_ as u8;

            for y_ in 1..256 {
                let y: u8 = y_ as u8;

                if (x as u16) * (y as u16) >= 256 { continue; }

                assert_eq!(
                    Some(x),
                    div_u8(x * y, y),
                    "div_u8 failed to divide {x} * {y} by {y}"
                )
            }
        }
    }

    #[test]
    fn coprime_test_div_u8() {
        use super::*;
        
        fn gcd(mut x: u8, mut y: u8) -> u8 {
            while y != 0 {
                let z = x % y;
                x = y;
                y = z;
            }
            x
        }

        for x_ in 1..256 {
            let x: u8 = x_ as u8;

            for y_ in 1..256 {
                let y: u8 = y_ as u8;

                if y % 128 == 0 && x % 128 != 0 { continue; }
                if y % 64 == 0 && x % 64 != 0 { continue; }
                if y % 32 == 0 && x % 32 != 0 { continue; }
                if y % 16 == 0 && x % 16 != 0 { continue; }
                if y % 8 == 0 && x % 8 != 0 { continue; }
                if y % 4 == 0 && x % 4 != 0 { continue; }
                if y % 2 == 0 && x % 2 != 0 { continue; }

                if gcd(x, y) != 0 { continue; }

                assert_eq!(
                    Some(x),
                    div_u8(x.wrapping_mul(y), y),
                    "div_u8 failed to divide {x} * {y} by {y}"
                )
            }
        }
    }

    #[test]
    fn case_test_div_u8() {
        use super::*;

        assert_eq!(Some(52), div_u8(4, 5));
    }
}

pub fn run_bfoptimised_block<
    Ask: FnMut() -> u8,
    Put: FnMut(u8) -> (),
    Get: FnMut(i32) -> u8,
    Set: FnMut(i32, u8) -> (),
    Clear: Fn() -> ()
>(
    ctx: &mut BFCtx<Ask, Put, Get, Set, Clear>,
    b: & OptimisedBlock
)
    -> bool
{
    match b {
        OptimisedBlock::Ask => { (ctx.set)(ctx.index, (ctx.ask)()); },
        OptimisedBlock::Put => (ctx.put)((ctx.get)(ctx.index)),
        OptimisedBlock::AtomicEffect(lines, offset) => {
            let mut buffer = HashMap::<i32, u8>::new();
            fn compute<
                'a,
                Ask: FnMut() -> u8,
                Put: FnMut(u8) -> (),
                Get: FnMut(i32) -> u8,
                Set: FnMut(i32, u8) -> (),
                Clear: Fn() -> ()
            >(
                ctx: &mut BFCtx<Ask, Put, Get, Set, Clear>,
                expr: &'a ProcExpr
            ) -> Option<u8> {
                match expr {
                    ProcExpr::Lit(x) => Some(*x),
                    ProcExpr::Reg(r) => Some((ctx.get)(ctx.index + r)),
                    ProcExpr::Add(a, b) =>
                        if let Some(x) = compute(ctx, a) {
                            if let Some(y) = compute(ctx, b) {
                                Some(
                                    x.wrapping_add(y)
                                )
                            } else {
                                None
                            }
                        } else {
                            None
                        },
                    ProcExpr::Mul(a, b) =>
                        if let Some(x) = compute(ctx, a) {
                            if let Some(y) = compute(ctx, b) {
                                Some(
                                    x.wrapping_mul(y)
                                )
                            } else {
                                None
                            }
                        } else {
                            None
                        },
                    ProcExpr::Into(a, b) => 
                    if let Some(x) = compute(ctx, a) {
                        if let Some(y) = compute(ctx, b) {
                            div_u8(y, x)
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                }
            }
            for (register, expr) in lines {
                if let Some(val) = compute(ctx, expr) {
                    buffer.insert(*register, val);
                } else {
                    return false;
                }
            }
            for (k, v) in buffer {
                (ctx.set)(ctx.index + k, v)
            }
            ctx.index += offset
        },
        OptimisedBlock::Loop(blocks) => {
            while (ctx.get)(ctx.index) != 0 {
                for b_ in blocks { if !run_bfoptimised_block(ctx, b_) { return false; } }
            }
        }
    };
    true
}

pub fn run_bfoptimised<
    Ask: FnMut() -> u8,
    Put: FnMut(u8) -> (),
    Get: FnMut(i32) -> u8,
    Set: FnMut(i32, u8) -> (),
    Clear: Fn() -> ()
>(
    ctx: &mut BFCtx<Ask, Put, Get, Set, Clear>,
    bs: Vec<OptimisedBlock>
)
    -> bool
{
    for b in bs {
        if !run_bfoptimised_block(ctx, &b) { return false; }
    };
    true
}

pub fn async_run_bfoptimised_block<
    'a,
    AskFuture: Future::<Output=u8>,
    Ask: FnMut() -> AskFuture,
    Put: FnMut(u8) -> (),
    Get: FnMut(i32) -> u8,
    Set: FnMut(i32, u8) -> (),
    Clear: Fn() -> ()
>(
    ctx: &'a mut AsyncBFCtx<AskFuture, Ask, Put, Get, Set, Clear>,
    b: &'a OptimisedBlock
)
    -> Pin<Box<dyn Future<Output=bool> + 'a>>
{
    Box::pin( async move {
        match b {
            OptimisedBlock::Ask => { (ctx.set)(ctx.index, (ctx.ask)().await); },
            OptimisedBlock::Put => (ctx.put)((ctx.get)(ctx.index)),
            OptimisedBlock::AtomicEffect(lines, offset) => {
                let mut buffer = HashMap::<i32, u8>::new();
                fn compute<
                    'a,
                    AskFuture: Future::<Output=u8>,
                    Ask: FnMut() -> AskFuture,
                    Put: FnMut(u8) -> (),
                    Get: FnMut(i32) -> u8,
                    Set: FnMut(i32, u8) -> (),
                    Clear: Fn() -> ()
                >(
                    ctx: &mut AsyncBFCtx<AskFuture, Ask, Put, Get, Set, Clear>,
                    expr: &'a ProcExpr
                ) -> Option<u8> {
                        match expr {
                            ProcExpr::Lit(x) => Some(*x),
                            ProcExpr::Reg(r) => Some((ctx.get)(ctx.index + r)),
                            ProcExpr::Add(a, b) =>
                                if let Some(x) = compute(ctx, a) {
                                    if let Some(y) = compute(ctx, b) {
                                        Some(
                                            x.wrapping_add(y)
                                        )
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                },
                            ProcExpr::Mul(a, b) =>
                                if let Some(x) = compute(ctx, a) {
                                    if let Some(y) = compute(ctx, b) {
                                        Some(
                                            x.wrapping_mul(y)
                                        )
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                },
                            ProcExpr::Into(a, b) => 
                            if let Some(x) = compute(ctx, a) {
                                if let Some(y) = compute(ctx, b) {
                                    div_u8(y, x)
                                } else {
                                    None
                                }
                            } else {
                                None
                            },
                        }
                }
                for (register, expr) in lines {
                    if let Some(val) = compute(ctx, expr) {
                        buffer.insert(*register, val);
                    } else {
                        return false;
                    }
                }
                for (k, v) in buffer {
                    (ctx.set)(ctx.index + k, v)
                }
                ctx.index += offset
            },
            OptimisedBlock::Loop(blocks) => {
                while (ctx.get)(ctx.index) != 0 {
                    for b_ in blocks { if !(async_run_bfoptimised_block(ctx, b_).await) { return false; } }
                }
            }
        };
        true
    })
}

pub async fn async_run_bfoptimised<
    AskFuture: Future::<Output=u8>,
    Ask: FnMut() -> AskFuture,
    Put: FnMut(u8) -> (),
    Get: FnMut(i32) -> u8,
    Set: FnMut(i32, u8) -> (),
    Clear: Fn() -> ()
>(
    ctx: &mut AsyncBFCtx<AskFuture, Ask, Put, Get, Set, Clear>,
    bs: Vec<OptimisedBlock>
)
    -> bool
{
    for b in bs {
        if !(async_run_bfoptimised_block(ctx, &b).await) { return false; }
    };
    true
}