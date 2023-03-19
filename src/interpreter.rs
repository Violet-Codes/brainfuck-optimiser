use std::collections::HashMap;

use super::BFRaw;

pub struct BFCtx<Ask: FnMut() -> u8, Put: FnMut(u8) -> ()> {
    pub index: i32,
    pub memory: HashMap<i32, u8>,
    pub ask: Ask,
    pub put: Put
}

pub fn run_bfraw_instruction<'a, Ask: FnMut() -> u8, Put: FnMut(u8) -> ()>(
    ctx: &mut BFCtx<Ask, Put>,
    i: &'a BFRaw
)
    -> ()
{
    match i {
        BFRaw::Lft => ctx.index -= 1,
        BFRaw::Rgh => ctx.index += 1,
        BFRaw::Inc => if let Some(mem) = ctx.memory.get_mut(& ctx.index) {
            if *mem == 255 {
                *mem = 0
            } else {
                *mem += 1
            }
        } else {
            ctx.memory.insert(ctx.index, 1);
        },
        BFRaw::Dec => if let Some(mem) = ctx.memory.get_mut(& ctx.index) {
            if *mem == 0 {
                *mem = 255
            } else {
                *mem -= 1
            }
        } else {
            ctx.memory.insert(ctx.index, 255);
        },
        BFRaw::Ask => { ctx.memory.insert(ctx.index, (ctx.ask)()); },
        BFRaw::Put => (ctx.put)(*ctx.memory.get(& ctx.index).unwrap_or(&0)),
        BFRaw::Loop(is) => while *ctx.memory.get(& ctx.index).unwrap_or(&0) != 0 {
            run_bfraw(ctx, is)
        },
    }
}

pub fn run_bfraw<'a, Ask: FnMut() -> u8, Put: FnMut(u8) -> ()>(
    ctx: &mut BFCtx<Ask, Put>,
    is: &'a Vec<BFRaw>
)
    -> ()
{
    is.iter().for_each(|i| run_bfraw_instruction(ctx, i))
}