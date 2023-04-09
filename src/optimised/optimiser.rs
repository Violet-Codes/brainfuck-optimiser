use std::collections::HashMap;

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
                        diff.into_iter().map(|(k, v)| ProcAssign{
                            register: k,
                            expr: ProcExpr::Add(
                                Box::new(ProcExpr::Reg(k)),
                                Box::new(ProcExpr::Lit(v))
                            )
                        }).collect(),
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
                        diff.into_iter().map(|(k, v)| ProcAssign{
                            register: k,
                            expr: ProcExpr::Add(
                                Box::new(ProcExpr::Reg(k)),
                                Box::new(ProcExpr::Lit(v))
                            )
                        }).collect(),
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

pub fn optimising_convert(
    raw: Vec<BFRaw>
)
    -> Vec<OptimisedBlock>
{
    convert(raw)
}