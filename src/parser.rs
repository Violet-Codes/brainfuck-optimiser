use nibbler::{ parser, builders::*, errors::*, monadic::*, combinators::*, alternative, select };

pub enum AST {
    Lft,
    Rgh,
    Inc,
    Dec,
    Ask,
    Put,
    Loop(Box<AST>)
}

pub const fn parse_any<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, Option<AST>]
{
    |iter| fmap(
        |_| None,
        predicate::<Iter, String, 1>(|_| true, |_| "unexpected end of file".to_string())
    )(iter)
}

pub const fn parse_lft<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, Option<AST>]
{
    |iter| fmap(
        |_| Some(AST::Lft),
        expect::<Iter, String, 1>(['<'], |_| "expected '<'".to_string())
    )(iter)
}

pub const fn parse_rgh<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, Option<AST>]
{
    |iter| fmap(
        |_| Some(AST::Rgh),
        expect::<Iter, String, 1>(['>'], |_| "expected '>'".to_string())
    )(iter)
}

pub const fn parse_inc<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, Option<AST>]
{
    |iter| fmap(
        |_| Some(AST::Inc),
        expect::<Iter, String, 1>(['+'], |_| "expected '+'".to_string())
    )(iter)
}

pub const fn parse_dec<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, Option<AST>]
{
    |iter| fmap(
        |_| Some(AST::Dec),
        expect::<Iter, String, 1>(['-'], |_| "expected '-'".to_string())
    )(iter)
}

pub const fn parse_ask<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, Option<AST>]
{
    |iter| fmap(
        |_| Some(AST::Ask),
        expect::<Iter, String, 1>([','], |_| "expected ','".to_string())
    )(iter)
}

pub const fn parse_put<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, Option<AST>]
{
    |iter| fmap(
        |_| Some(AST::Put),
        expect::<Iter, String, 1>(['.'], |_| "expected '.'".to_string())
    )(iter)
}

pub const fn parse_loop_start<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, ()]
{
    |iter| fmap(
        |_| (),
        expect::<Iter, String, 1>(['['], |_| "expected '['".to_string())
    )(iter)
}

pub const fn parse_loop_end<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, ()]
{
    |iter| fmap(
        |_| (),
        expect::<Iter, String, 1>([']'], |_| "expected ']'".to_string())
    )(iter)
}

pub const fn parse_instruction<Iter: Iterator<Item = char>>()
    -> parser![Iter, String, Option<AST>]
{
    |iter| fmap_err(
        |_| "expected '<', '>', '+', '-', ',', '.' or comment".to_string(),
        alternative!(
            wrap_err(parse_lft()),
            wrap_err(parse_rgh()),
            wrap_err(parse_inc()),
            wrap_err(parse_dec()),
            wrap_err(parse_ask()),
            wrap_err(parse_put()),
            wrap_err(parse_any())
        )
    )(iter)
}

pub const fn parse_loop<Iter: Iterator<Item = char>>()
    -> parser![Iter, Vec<String>, Vec<Option<AST>>]
{
    |iter| select!(
        wrap_err(parse_loop_start()),
        =>  fmap(
            |(v, _e)| v,
            least_till(
                parse_instruction(),
                parse_loop_end()
            )
        )
    )(iter)
}
