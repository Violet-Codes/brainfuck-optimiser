use nibbler::{ parser, builders::*, errors::*, monadic::*, combinators::*, alternative, select };

use super::*;

#[derive(Debug, Clone)]
pub struct TextInfo{
    pub line: usize,
    pub index: usize
}

#[derive(Clone)]
pub struct TextIter<Iter>{
    pub iter: Iter,
    pub line: usize,
    pub index: usize
}

impl<Iter: Iterator<Item = char>> Iterator for TextIter<Iter> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Option::None => Option::None,
            Option::Some('\n') => { self.line += 1; self.index = 0; Option::Some('\n') },
            Option::Some(c) => { self.index += 1; Option::Some(c) }
        }
    }
}

macro_rules! info_getter {
    () => (
        |iter_| TextInfo{ line: iter_.line, index: iter_.index }
    );
}

macro_rules! msg {
    ($s:expr) => (
        |iter_: & TextIter<Iter>| ParseError::Message(($s).to_string(), TextInfo{ line: iter_.line, index: iter_.index })
    );
}

pub const fn parse_comment<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>]
{
    |iter| fmap(
        |_| None,
        predicate::<TextIter<Iter>, ParseError<TextInfo>, 1>(
            |s|
                *s != [':'] &&
                *s != ['<'] &&
                *s != ['>'] &&
                *s != ['+'] &&
                *s != ['-'] &&
                *s != [','] &&
                *s != ['.'] &&
                *s != ['['] &&
                *s != [']'],
            |_| ParseError::Silent
        )
    )(iter)
}

pub const fn parse_lft<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>]
{
    |iter| fmap(
        |_| Some(BFRaw::Lft),
        expect::<TextIter<Iter>, ParseError<TextInfo>, 1>(
            ['<'],
            msg!("'<'")
        )
    )(iter)
}

pub const fn parse_rgh<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>]
{
    |iter| fmap(
        |_| Some(BFRaw::Rgh),
        expect::<TextIter<Iter>, ParseError<TextInfo>, 1>(
            ['>'],
            msg!("'>'")
        )
    )(iter)
}

pub const fn parse_inc<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>]
{
    |iter| fmap(
        |_| Some(BFRaw::Inc),
        expect::<TextIter<Iter>, ParseError<TextInfo>, 1>(
            ['+'],
            msg!("'+'")
        )
    )(iter)
}

pub const fn parse_dec<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>]
{
    |iter| fmap(
        |_| Some(BFRaw::Dec),
        expect::<TextIter<Iter>, ParseError<TextInfo>, 1>(
            ['-'],
            msg!("'-'")
        )
    )(iter)
}

pub const fn parse_ask<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>]
{
    |iter| fmap(
        |_| Some(BFRaw::Ask),
        expect::<TextIter<Iter>, ParseError<TextInfo>, 1>(
            [','],
            msg!("','")
        )
    )(iter)
}

pub const fn parse_put<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>]
{
    |iter| fmap(
        |_| Some(BFRaw::Put),
        expect::<TextIter<Iter>, ParseError<TextInfo>, 1>(
            ['.'],
            msg!("'.'")
        )
    )(iter)
}

pub const fn parse_loop_start<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, ()]
{
    |iter| fmap(
        |_| (),
        expect::<TextIter<Iter>, ParseError<TextInfo>, 1>(
            ['['],
            msg!("'['")
        )
    )(iter)
}

pub const fn parse_loop_end<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, ()]
{
    |iter| fmap(
        |_| (),
        expect::<TextIter<Iter>, ParseError<TextInfo>, 1>(
            [']'],
            msg!("']'")
        )
    )(iter)
}

pub const fn parse_instruction<Iter: Iterator<Item = char> + Clone>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>]
{
    |iter| display_fst_nonsilent(
        alternative!(
            try_parse(parse_loop()),
            select!(
                silence(
                    negate(
                        try_parse(
                            eos::<TextIter<Iter>, ParseError<TextInfo>>(
                                msg!("end of stream")
                            )
                        )
                    )
                ),
                => fail::<TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>>(msg!("expression"))
            ),
            silence(
                alternative!(
                    try_parse(parse_lft()),
                    try_parse(parse_rgh()),
                    try_parse(parse_inc()),
                    try_parse(parse_dec()),
                    try_parse(parse_ask()),
                    try_parse(parse_put()),
                    parse_comment()
                )
            )
        )
    )(iter)
}

pub const fn parse_loop<Iter: Iterator<Item = char> + Clone>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Option<BFRaw>]
{
    |iter| label("loop".to_string(), info_getter!(), select!(
        silence(parse_loop_start()),
        =>  display_fst_nonsilent(
            fmap(
                |(v, _e)| Option::Some(BFRaw::Loop(v.into_iter().filter_map(|op_ast| op_ast).collect())),
                most_till(
                    try_parse(parse_instruction()),
                    parse_loop_end()
                )
            )
        )
    ))(iter)
}

pub const fn parse_program<Iter: Iterator<Item = char> + Clone>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, Vec<BFRaw>]
{
    |iter| display_lst_nonsilent(
        fmap(
            |(v, _e)| v.into_iter().filter_map(|op_ast| op_ast).collect(),
            least_till(
                parse_instruction(),
                try_parse(
                    eos::<TextIter<Iter>, ParseError<TextInfo>>(
                        msg!("end of stream")
                    )
                )
            )
        )
    )(iter)
}

pub enum BFCMD {
    Exit,
    Read(i32),
    Clear,
    Help,
    Find,
    Move(i32)
}

pub const fn parse_exit<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, BFCMD]
{
    |iter| fmap(
        |_| BFCMD::Exit,
        expect::<TextIter<Iter>, ParseError<TextInfo>, 2>(
            [':', 'q'],
            msg!("':q'")
        )
    )(iter)
}

pub const fn parse_read<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, BFCMD]
{
    |iter| fmap(
        |x| BFCMD::Read(x),
        select!(
            silence(expect::<TextIter<Iter>, ParseError<TextInfo>, 2>(
                [':', 'r'],
                msg!("':r'")
            )),
            => |iter_| {
                let err = msg!("number")(iter_);
                str::parse::<i32>(iter_.collect::<String>().as_str().trim()).map_err(|_| err)
            }
        )
    )(iter)
}

pub const fn parse_clear<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, BFCMD]
{
    |iter| fmap(
        |_| BFCMD::Clear,
        expect::<TextIter<Iter>, ParseError<TextInfo>, 2>(
            [':', 'c'],
            msg!("':c'")
        )
    )(iter)
}

pub const fn parse_help<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, BFCMD]
{
    |iter| fmap(
        |_| BFCMD::Help,
        expect::<TextIter<Iter>, ParseError<TextInfo>, 2>(
            [':', 'h'],
            msg!("':h'")
        )
    )(iter)
}

pub const fn parse_find<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, BFCMD]
{
    |iter| fmap(
        |_| BFCMD::Find,
        expect::<TextIter<Iter>, ParseError<TextInfo>, 2>(
            [':', 'f'],
            msg!("':f'")
        )
    )(iter)
}

pub const fn parse_move<Iter: Iterator<Item = char>>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, BFCMD]
{
    |iter| fmap(
        |x| BFCMD::Move(x),
        select!(
            silence(expect::<TextIter<Iter>, ParseError<TextInfo>, 2>(
                [':', 'm'],
                msg!("':m'")
            )),
            => |iter_| {
                let err = msg!("number")(iter_);
                str::parse::<i32>(iter_.collect::<String>().as_str().trim()).map_err(|_| err)
            }
        )
    )(iter)
}

pub const fn parse_bfcmd<Iter: Iterator<Item = char> + Clone>()
    -> parser![TextIter<Iter>, ParseError<TextInfo>, BFCMD]
{
    |iter| display_fst_nonsilent(
        alternative!(
            silence(try_parse(parse_exit())),
            try_parse(parse_read()),
            silence(try_parse(parse_clear())),
            silence(try_parse(parse_help())),
            silence(try_parse(parse_find())),
            try_parse(parse_move()),
            fail(msg!("command (e.g. ':h')"))
        )
    )(iter)
}