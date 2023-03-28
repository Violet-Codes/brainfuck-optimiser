use nibbler::errors::show_error;
use crate::{ parser::*, interpreter::* };

pub struct ConsoleInteractor<
    ReadLn: FnMut(String) -> String,
    WriteLn: FnMut(String) -> (),
    WriteErrLn: FnMut(String) -> (),
    DisplayHelp: FnMut() -> ()
>{
    pub readln: ReadLn,
    pub writeln: WriteLn,
    pub write_errln: WriteErrLn,
    pub display_help: DisplayHelp
}

pub fn REP<
    ReadLn: FnMut(String) -> String,
    WriteLn: FnMut(String) -> (),
    WriteErrLn: FnMut(String) -> (),
    DisplayHelp: FnMut() -> (),
    Ask: FnMut() -> u8,
    Put: FnMut(u8) -> ()
>(
    console_interactor: &mut ConsoleInteractor<ReadLn, WriteLn, WriteErrLn, DisplayHelp>,
    ctx: &mut BFCtx<Ask, Put>
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
            Ok(BFCMD::Read(x)) => (console_interactor.writeln)(format!("#{x}: {}", ctx.memory.get(& x).unwrap_or(& 0))),
            Ok(BFCMD::Clear) => ctx.memory.clear(),
            Ok(BFCMD::Help) => (console_interactor.display_help)(),
            Ok(BFCMD::Find) => println!("head: #{}", ctx.index),
            Ok(BFCMD::Move(x)) => ctx.index = x,
            Err(err) => (console_interactor.write_errln)(format!("{}\n...whilst parsing instruction", show_error("".to_string(), & show_info, err))),
        };
    } else {
        match parse_program()(&mut iter) {
            Ok(is) => run_bfraw(ctx, & is),
            Err(err) => (console_interactor.write_errln)(format!("{}\n...whilst parsing input", show_error("".to_string(), & show_info, err))),
        };
    };
    true
}