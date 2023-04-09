use std::collections::HashMap;

use super::super::interpreter::*;
use super::*;

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
    -> ()
{
    match b {
        OptimisedBlock::AtomicEffect(lines, offset) => {
            let mut lookup = HashMap::<& ProcExpr, u8>::new();
            fn resolve_inner<
                'a,
                Ask: FnMut() -> u8,
                Put: FnMut(u8) -> (),
                Get: FnMut(i32) -> u8,
                Set: FnMut(i32, u8) -> (),
                Clear: Fn() -> ()
            >(
                lookup: &mut HashMap::<&'a ProcExpr, u8>,
                ctx: &mut BFCtx<Ask, Put, Get, Set, Clear>,
                expr: &'a ProcExpr
            ) -> u8 {
                if let Some(x) = lookup.get(expr) {
                    *x
                } else {
                    let val = match expr {
                        ProcExpr::Lit(x) => *x,
                        ProcExpr::Reg(r) => (ctx.get)(ctx.index + r),
                        ProcExpr::Add(a, b) =>
                            (resolve_inner(lookup, ctx, a))
                            .wrapping_add(resolve_inner(lookup, ctx, b)),
                        ProcExpr::Mul(a, b) =>
                            (resolve_inner(lookup, ctx, a))
                            .wrapping_add(resolve_inner(lookup, ctx, b)),
                        ProcExpr::Into(_, _) => todo!(),
                    };
                    lookup.insert(expr, val);
                    val
                }
            }
        },
        OptimisedBlock::Loop(blocks) => for b_ in blocks { run_bfoptimised_block(ctx, b_) },
    }
}