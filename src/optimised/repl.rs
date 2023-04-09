use std::future::Future;

use nibbler::errors::show_error;
use crate::{ parser::*, interpreter::* };

use super::{ *, optimiser::*, interpreter::* };

pub struct ConsoleInteractor<
    ReadLn: FnMut(String) -> String,
    WriteLn: FnMut(String) -> (),
    WriteErrLn: FnMut(String) -> (),
    DisplayHelp: FnMut() -> (),
    DisplayOptimisation: FnMut(& Vec<OptimisedBlock>) -> () // may need to take in a different type
>{
    pub readln: ReadLn,
    pub writeln: WriteLn,
    pub write_errln: WriteErrLn,
    pub display_help: DisplayHelp,
    pub display_optimisation: DisplayOptimisation
}

pub fn rep<
    ReadLn: FnMut(String) -> String,
    WriteLn: FnMut(String) -> (),
    WriteErrLn: FnMut(String) -> (),
    DisplayHelp: FnMut() -> (),
    DisplayOptimisation: FnMut(& Vec<OptimisedBlock>) -> (),
    Ask: FnMut() -> u8,
    Put: FnMut(u8) -> (),
    Get: FnMut(i32) -> u8,
    Set: FnMut(i32, u8) -> (),
    Clear: Fn() -> ()
>(
    console_interactor: &mut ConsoleInteractor<ReadLn, WriteLn, WriteErrLn, DisplayHelp, DisplayOptimisation>,
    ctx: &mut BFCtx<Ask, Put, Get, Set, Clear>
)
    -> bool
{
    let show_info = |info: TextInfo| format!("at {}:{}", info.line, info.index);
    let s = (console_interactor.readln)("$ ".to_string());
    let mut iter = TextIter{
        iter: s.chars(),
        line: 0,
        index: 0
    };
    if s.chars().nth(0) == Some(':') {
        match parse_bfcmd()(&mut iter) {
            Ok(BFCMD::Exit) => return false,
            Ok(BFCMD::Read(x)) => (console_interactor.writeln)(format!("#{x}: {}", (ctx.get)(x))),
            Ok(BFCMD::Clear) => (ctx.clear)(),
            Ok(BFCMD::Help) => (console_interactor.display_help)(),
            Ok(BFCMD::Find) => (console_interactor.writeln)(format!("head: #{}", ctx.index)),
            Ok(BFCMD::Move(x)) => ctx.index = x,
            Err(err) => (console_interactor.write_errln)(format!("{}\n...whilst parsing instruction", show_error("".to_string(), & show_info, err))),
        };
    } else {
        match parse_program()(&mut iter) {
            Ok(is) => {
                let optimised = optimising_convert(is);
                (console_interactor.display_optimisation)(& optimised);
                if !run_bfoptimised(ctx, optimised) {
                    (console_interactor.write_errln)("aborted non-halting loop".to_string())
                }
            },
            Err(err) => (console_interactor.write_errln)(format!("{}\n...whilst parsing input", show_error("".to_string(), & show_info, err))),
        };
    };
    true
}

pub struct AsyncConsoleInteractor<
    ReadLnFuture: Future::<Output=String>,
    ReadLn: FnMut(String) -> ReadLnFuture,
    WriteLn: FnMut(String) -> (),
    WriteErrLn: FnMut(String) -> (),
    DisplayHelp: FnMut() -> (),
    DisplayOptimisation: FnMut(& Vec<OptimisedBlock>) -> () // may need to take in a different type
>{
    pub readln: ReadLn,
    pub writeln: WriteLn,
    pub write_errln: WriteErrLn,
    pub display_help: DisplayHelp,
    pub display_optimisation: DisplayOptimisation
}

pub async fn async_rep<
    ReadLnFuture: Future::<Output=String>,
    ReadLn: FnMut(String) -> ReadLnFuture,
    WriteLn: FnMut(String) -> (),
    WriteErrLn: FnMut(String) -> (),
    DisplayHelp: FnMut() -> (),
    DisplayOptimisation: FnMut(& Vec<OptimisedBlock>) -> (),
    AskFuture: Future::<Output=u8>,
    Ask: FnMut() -> AskFuture,
    Put: FnMut(u8) -> (),
    Get: FnMut(i32) -> u8,
    Set: FnMut(i32, u8) -> (),
    Clear: Fn() -> ()
>(
    console_interactor: &mut AsyncConsoleInteractor<ReadLnFuture, ReadLn, WriteLn, WriteErrLn, DisplayHelp, DisplayOptimisation>,
    ctx: &mut AsyncBFCtx<AskFuture, Ask, Put, Get, Set, Clear>
)
    -> bool
{
    let show_info = |info: TextInfo| format!("at {}:{}", info.line, info.index);
    let s = (console_interactor.readln)("$ ".to_string()).await;
    let mut iter = TextIter{
        iter: s.chars(),
        line: 0,
        index: 0
    };
    if s.chars().nth(0) == Some(':') {
        match parse_bfcmd()(&mut iter) {
            Ok(BFCMD::Exit) => return false,
            Ok(BFCMD::Read(x)) => (console_interactor.writeln)(format!("#{x}: {}", (ctx.get)(x))),
            Ok(BFCMD::Clear) => (ctx.clear)(),
            Ok(BFCMD::Help) => (console_interactor.display_help)(),
            Ok(BFCMD::Find) => (console_interactor.writeln)(format!("head: #{}", ctx.index)),
            Ok(BFCMD::Move(x)) => ctx.index = x,
            Err(err) => (console_interactor.write_errln)(format!("{}\n...whilst parsing instruction", show_error("".to_string(), & show_info, err))),
        };
    } else {
        match parse_program()(&mut iter) {
            Ok(is) => {
                let optimised = optimising_convert(is);
                (console_interactor.display_optimisation)(& optimised);
                if !(async_run_bfoptimised(ctx, optimised).await) {
                    (console_interactor.write_errln)("aborted non-halting loop".to_string())
                }
            },
            Err(err) => (console_interactor.write_errln)(format!("{}\n...whilst parsing input", show_error("".to_string(), & show_info, err))),
        };
    };
    true
}