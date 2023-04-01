use super::BFRaw;

pub struct BFCtx<
    Ask: FnMut() -> u8,
    Put: FnMut(u8) -> (),
    Get: FnMut(i32) -> u8,
    Set: FnMut(i32, u8) -> (),
    Clear: Fn() -> ()
>{
    pub index: i32,
    pub get: Get,
    pub set: Set,
    pub ask: Ask,
    pub put: Put,
    pub clear: Clear
}

pub fn run_bfraw_instruction<
    Ask: FnMut() -> u8,
    Put: FnMut(u8) -> (),
    Get: FnMut(i32) -> u8,
    Set: FnMut(i32, u8) -> (),
    Clear: Fn() -> ()
>(
    ctx: &mut BFCtx<Ask, Put, Get, Set, Clear>,
    i: & BFRaw
)
    -> ()
{
    match i {
        BFRaw::Lft => ctx.index -= 1,
        BFRaw::Rgh => ctx.index += 1,
        BFRaw::Inc => {
            let mem = (ctx.get)(ctx.index);
            if mem == 255 {
                (ctx.set)(ctx.index, 0)
            } else {
                (ctx.set)(ctx.index, mem + 1)
            }
        },
        BFRaw::Dec => {
            let mem = (ctx.get)(ctx.index);
            if mem == 0 {
                (ctx.set)(ctx.index, 255)
            } else {
                (ctx.set)(ctx.index, mem - 1)
            }
        },
        BFRaw::Ask => { (ctx.set)(ctx.index, (ctx.ask)()); },
        BFRaw::Put => (ctx.put)((ctx.get)(ctx.index)),
        BFRaw::Loop(is) => while (ctx.get)(ctx.index) != 0 {
            run_bfraw(ctx, is)
        },
    }
}

pub fn run_bfraw<
    Ask: FnMut() -> u8,
    Put: FnMut(u8) -> (),
    Get: FnMut(i32) -> u8,
    Set: FnMut(i32, u8) -> (),
    Clear: Fn() -> ()
>(
    ctx: &mut BFCtx<Ask, Put, Get, Set, Clear>,
    is: & Vec<BFRaw>
)
    -> ()
{
    is.iter().for_each(|i| run_bfraw_instruction(ctx, i))
}