use std::collections::HashMap;

use super::super::interpreter::*;
use super::*;

// https://www.wolframalpha.com/input?i=x+*+3+%3D+1+mod+256

fn inv256(x: u8) -> Option<u8> {
    match x {
        1 => Some(1),
        3 => Some(171),
        5 => Some(205),
        7 => Some(183),
        9 => Some(57),
        11 => Some(163),
        13 => Some(197),
        15 => Some(239),
        17 => Some(241),
        19 => Some(27),
        21 => Some(61),
        23 => Some(167),
        25 => Some(41),
        27 => Some(19),
        29 => Some(53),
        31 => Some(223),
        33 => Some(225),
        35 => Some(139),
        37 => Some(173),
        39 => Some(151),
        41 => Some(25),
        43 => Some(131),
        45 => Some(165),
        47 => Some(207),
        49 => Some(209),
        51 => Some(251),
        53 => Some(29),
        55 => Some(135),
        57 => Some(9),
        59 => Some(243),
        61 => Some(21),
        63 => Some(191),
        65 => Some(193),
        67 => Some(107),
        69 => Some(141),
        71 => Some(119),
        73 => Some(249),
        75 => Some(99),
        77 => Some(133),
        79 => Some(175),
        81 => Some(177),
        83 => Some(219),
        85 => Some(253),
        87 => Some(103),
        89 => Some(233),
        91 => Some(211),
        93 => Some(245),
        95 => Some(159),
        97 => Some(161),
        99 => Some(75),
        101 => Some(109),
        103 => Some(87),
        105 => Some(217),
        107 => Some(67),
        109 => Some(101),
        111 => Some(143),
        113 => Some(145),
        115 => Some(187),
        117 => Some(221),
        119 => Some(71),
        121 => Some(201),
        123 => Some(179),
        125 => Some(213),
        127 => Some(127),
        129 => Some(129),
        131 => Some(43),
        133 => Some(77),
        135 => Some(55),
        137 => Some(185),
        139 => Some(35),
        141 => Some(69),
        143 => Some(111),
        145 => Some(113),
        147 => Some(155),
        149 => Some(189),
        151 => Some(39),
        153 => Some(169),
        155 => Some(147),
        157 => Some(181),
        159 => Some(95),
        161 => Some(97),
        163 => Some(11),
        165 => Some(45),
        167 => Some(23),
        169 => Some(153),
        171 => Some(3),
        173 => Some(37),
        175 => Some(79),
        177 => Some(81),
        179 => Some(123),
        181 => Some(157),
        183 => Some(7),
        185 => Some(137),
        187 => Some(115),
        189 => Some(149),
        191 => Some(63),
        193 => Some(65),
        195 => Some(235),
        197 => Some(13),
        199 => Some(247),
        201 => Some(121),
        203 => Some(227),
        205 => Some(5),
        207 => Some(47),
        209 => Some(49),
        211 => Some(91),
        213 => Some(125),
        215 => Some(231),
        217 => Some(105),
        219 => Some(83),
        221 => Some(117),
        223 => Some(31),
        225 => Some(33),
        227 => Some(203),
        229 => Some(237),
        231 => Some(215),
        233 => Some(89),
        235 => Some(195),
        237 => Some(229),
        239 => Some(15),
        241 => Some(17),
        243 => Some(59),
        245 => Some(93),
        247 => Some(199),
        249 => Some(73),
        251 => Some(51),
        253 => Some(85),
        255 => Some(255),
        _ => None
    }
}

fn inv128(x: u8) -> Option<u8> {
    match x {
        1 => Some(1),
        3 => Some(43),
        5 => Some(77),
        7 => Some(55),
        9 => Some(57),
        11 => Some(35),
        13 => Some(69),
        15 => Some(111),
        17 => Some(113),
        19 => Some(27),
        21 => Some(61),
        23 => Some(39),
        25 => Some(41),
        27 => Some(19),
        29 => Some(53),
        31 => Some(95),
        33 => Some(97),
        35 => Some(11),
        37 => Some(45),
        39 => Some(23),
        41 => Some(25),
        43 => Some(3),
        45 => Some(37),
        47 => Some(79),
        49 => Some(81),
        51 => Some(123),
        53 => Some(29),
        55 => Some(7),
        57 => Some(9),
        59 => Some(115),
        61 => Some(21),
        63 => Some(63),
        65 => Some(65),
        67 => Some(107),
        69 => Some(13),
        71 => Some(119),
        73 => Some(121),
        75 => Some(99),
        77 => Some(5),
        79 => Some(47),
        81 => Some(49),
        83 => Some(91),
        85 => Some(125),
        87 => Some(103),
        89 => Some(105),
        91 => Some(83),
        93 => Some(117),
        95 => Some(31),
        97 => Some(33),
        99 => Some(75),
        101 => Some(109),
        103 => Some(87),
        105 => Some(89),
        107 => Some(67),
        109 => Some(101),
        111 => Some(15),
        113 => Some(17),
        115 => Some(59),
        117 => Some(93),
        119 => Some(71),
        121 => Some(73),
        123 => Some(51),
        125 => Some(85),
        127 => Some(127),
        _ => None
    }
}

fn div_u8(mut x: u8, mut y: u8) -> Option<u8> {
    if x == 0 { return Some(0); }
    if y == 0 { return None; }
    // x, y in 0..256
    if y % 2 != 0 { return inv256(y).map(|y_| x.wrapping_mul(y_)); }
    if x % 2 != 0 { return None; }
    x >>= 2;
    y >>= 2;
    // x, y in 0..128
    if y % 2 != 0 { return inv128(y).map(|y_| x.wrapping_mul(y_) % 128); }
    if x % 2 != 0 { return None; }
    x >>= 2;
    y >>= 2;
    // x, y in 0..64
    // if y % 2 != 0 { return inv64(y).map(|y_| x.wrapping_mul(y_) % 64); }
    // if x % 2 != 0 { return None; }
    x >>= 2;
    y >>= 2;
    // x, y in 0..32
    // if y % 2 != 0 { return inv32(y).map(|y_| x.wrapping_mul(y_) % 32); }
    // if x % 2 != 0 { return None; }
    x >>= 2;
    y >>= 2;
    // x, y in 0..16
    // if y % 2 != 0 { return inv16(y).map(|y_| x.wrapping_mul(y_) % 16); }
    // if x % 2 != 0 { return None; }
    x >>= 2;
    y >>= 2;
    // x, y in 0..8
    // if y % 2 != 0 { return inv8(y).map(|y_| x.wrapping_mul(y_) % 8); }
    // if x % 2 != 0 { return None; }
    x >>= 2;
    y >>= 2;
    // x, y in 0..4
    // if y % 2 != 0 { return inv4(y).map(|y_| x.wrapping_mul(y_) % 4); }
    // if x % 2 != 0 { return None; }
    x >>= 2;
    y >>= 2;
    // x, y in 0..2
    // x == y == 1
    Some(1)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_inv256() {
        use crate::optimised::repl::*;

        for x_ in 0..256 {
            let x: u8 = x_ as u8;
            assert_eq!(
                if x % 2 == 0 {
                    None
                } else {
                    Some(1)
                },
                inv256(x).map(|val| x.wrapping_mul(val)),
                "inv256 failed to invert {x}"
            );
        }
    }

    #[test]
    fn test_inv128() {
        use crate::optimised::repl::*;

        for x_ in 0..128 {
            let x: u8 = x_ as u8;
            assert_eq!(
                if x % 2 == 0 {
                    None
                } else {
                    Some(1)
                },
                inv128(x).map(|val| x.wrapping_mul(val) % 128),
                "inv128 failed to invert {x}"
            );
        }
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