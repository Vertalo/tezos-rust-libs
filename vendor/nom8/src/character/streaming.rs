//! Character specific parsers and combinators, streaming version
//!
//! Functions recognizing specific characters

#![allow(deprecated)]

use crate::branch::alt;
use crate::combinator::opt;
use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::input::{
  AsChar, FindToken, InputIter, InputLength, InputTake, InputTakeAtPosition, IntoOutput, Slice,
};
use crate::input::{Compare, CompareResult};
use crate::lib::std::ops::{Range, RangeFrom, RangeTo};
use crate::IntoOutputIResult as _;
use crate::{Err, IResult, Needed};

/// Recognizes one character.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{Err, error::{ErrorKind, Error}, Needed, IResult};
/// # use nom8::character::streaming::char;
/// fn parser(i: &str) -> IResult<&str, char> {
///     char('a')(i)
/// }
/// assert_eq!(parser("abc"), Ok(("bc", 'a')));
/// assert_eq!(parser("bc"), Err(Err::Error(Error::new("bc", ErrorKind::Char))));
/// assert_eq!(parser(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::bytes::one_of`][crate::bytes::one_of] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::bytes::one_of` with input wrapped in `nom8::input::Streaming`"
)]
pub fn char<I, Error: ParseError<I>>(c: char) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + InputLength,
  <I as InputIter>::Item: AsChar,
{
  move |i: I| char_internal(i, c)
}

pub(crate) fn char_internal<I, Error: ParseError<I>>(i: I, c: char) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + InputLength,
  <I as InputIter>::Item: AsChar,
{
  match (i).iter_elements().next().map(|t| {
    let b = t.as_char() == c;
    (&c, b)
  }) {
    None => Err(Err::Incomplete(Needed::new(c.len() - i.input_len()))),
    Some((_, false)) => Err(Err::Error(Error::from_char(i, c))),
    Some((c, true)) => Ok((i.slice(c.len()..), c.as_char())),
  }
}

/// Recognizes one character and checks that it satisfies a predicate
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{Err, error::{ErrorKind, Error}, Needed, IResult};
/// # use nom8::character::streaming::satisfy;
/// fn parser(i: &str) -> IResult<&str, char> {
///     satisfy(|c| c == 'a' || c == 'b')(i)
/// }
/// assert_eq!(parser("abc"), Ok(("bc", 'a')));
/// assert_eq!(parser("cd"), Err(Err::Error(Error::new("cd", ErrorKind::Satisfy))));
/// assert_eq!(parser(""), Err(Err::Incomplete(Needed::Unknown)));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::bytes::one_of`][crate::bytes::one_of] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::bytes::one_of` with input wrapped in `nom8::input::Streaming`"
)]
pub fn satisfy<F, I, Error: ParseError<I>>(cond: F) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter,
  <I as InputIter>::Item: AsChar,
  F: Fn(char) -> bool,
{
  move |i: I| satisfy_internal(i, &cond)
}

pub(crate) fn satisfy_internal<F, I, Error: ParseError<I>>(
  i: I,
  cond: &F,
) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter,
  <I as InputIter>::Item: AsChar,
  F: Fn(char) -> bool,
{
  match (i).iter_elements().next().map(|t| {
    let c = t.as_char();
    let b = cond(c);
    (c, b)
  }) {
    None => Err(Err::Incomplete(Needed::Unknown)),
    Some((_, false)) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Satisfy))),
    Some((c, true)) => Ok((i.slice(c.len()..), c)),
  }
}

/// Recognizes one of the provided characters.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, Needed};
/// # use nom8::character::streaming::one_of;
/// assert_eq!(one_of::<_, _, (_, ErrorKind)>("abc")("b"), Ok(("", 'b')));
/// assert_eq!(one_of::<_, _, (_, ErrorKind)>("a")("bc"), Err(Err::Error(("bc", ErrorKind::OneOf))));
/// assert_eq!(one_of::<_, _, (_, ErrorKind)>("a")(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::bytes::one_of`][crate::bytes::one_of] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::bytes::one_of` with input wrapped in `nom8::input::Streaming`"
)]
pub fn one_of<I, T, Error: ParseError<I>>(list: T) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + InputLength,
  <I as InputIter>::Item: AsChar + Copy,
  T: FindToken<<I as InputIter>::Item>,
{
  move |i: I| crate::bytes::streaming::one_of_internal(i, &list).map(|(i, c)| (i, c.as_char()))
}

/// Recognizes a character that is not in the provided characters.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, Needed};
/// # use nom8::character::streaming::none_of;
/// assert_eq!(none_of::<_, _, (_, ErrorKind)>("abc")("z"), Ok(("", 'z')));
/// assert_eq!(none_of::<_, _, (_, ErrorKind)>("ab")("a"), Err(Err::Error(("a", ErrorKind::NoneOf))));
/// assert_eq!(none_of::<_, _, (_, ErrorKind)>("a")(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::bytes::none_of`][crate::bytes::none_of] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::bytes::none_of` with input wrapped in `nom8::input::Streaming`"
)]
pub fn none_of<I, T, Error: ParseError<I>>(list: T) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputLength + InputIter,
  <I as InputIter>::Item: AsChar + Copy,
  T: FindToken<<I as InputIter>::Item>,
{
  move |i: I| crate::bytes::streaming::none_of_internal(i, &list).map(|(i, c)| (i, c.as_char()))
}

/// Recognizes the string "\r\n".
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::crlf;
/// assert_eq!(crlf::<_, (_, ErrorKind)>("\r\nc"), Ok(("c", "\r\n")));
/// assert_eq!(crlf::<_, (_, ErrorKind)>("ab\r\nc"), Err(Err::Error(("ab\r\nc", ErrorKind::CrLf))));
/// assert_eq!(crlf::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(2))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::crlf`][crate::character::crlf] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::crlf` with input wrapped in `nom8::input::Streaming`"
)]
pub fn crlf<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
  T: InputIter,
  T: IntoOutput,
  T: Compare<&'static str>,
{
  match input.compare("\r\n") {
    //FIXME: is this the right index?
    CompareResult::Ok => Ok((input.slice(2..), input.slice(0..2))).into_output(),
    CompareResult::Incomplete => Err(Err::Incomplete(Needed::new(2))),
    CompareResult::Error => {
      let e: ErrorKind = ErrorKind::CrLf;
      Err(Err::Error(E::from_error_kind(input, e)))
    }
  }
}

/// Recognizes a string of any char except '\r\n' or '\n'.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{Err, error::{Error, ErrorKind}, IResult, Needed};
/// # use nom8::character::streaming::not_line_ending;
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>("ab\r\nc"), Ok(("\r\nc", "ab")));
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>("abc"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>("a\rb\nc"), Err(Err::Error(("a\rb\nc", ErrorKind::Tag ))));
/// assert_eq!(not_line_ending::<_, (_, ErrorKind)>("a\rbc"), Err(Err::Error(("a\rbc", ErrorKind::Tag ))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::not_line_ending`][crate::character::not_line_ending] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::not_line_ending` with input wrapped in `nom8::input::Streaming`"
)]
pub fn not_line_ending<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
  T: InputIter + InputLength,
  T: IntoOutput,
  T: Compare<&'static str>,
  <T as InputIter>::Item: AsChar,
  <T as InputIter>::Item: AsChar,
{
  match input.position(|item| {
    let c = item.as_char();
    c == '\r' || c == '\n'
  }) {
    None => Err(Err::Incomplete(Needed::Unknown)),
    Some(index) => {
      let mut it = input.slice(index..).iter_elements();
      let nth = it.next().unwrap().as_char();
      if nth == '\r' {
        let sliced = input.slice(index..);
        let comp = sliced.compare("\r\n");
        match comp {
          //FIXME: calculate the right index
          CompareResult::Incomplete => Err(Err::Incomplete(Needed::Unknown)),
          CompareResult::Error => {
            let e: ErrorKind = ErrorKind::Tag;
            Err(Err::Error(E::from_error_kind(input, e)))
          }
          CompareResult::Ok => Ok((input.slice(index..), input.slice(..index))).into_output(),
        }
      } else {
        Ok((input.slice(index..), input.slice(..index))).into_output()
      }
    }
  }
}

/// Recognizes an end of line (both '\n' and '\r\n').
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::line_ending;
/// assert_eq!(line_ending::<_, (_, ErrorKind)>("\r\nc"), Ok(("c", "\r\n")));
/// assert_eq!(line_ending::<_, (_, ErrorKind)>("ab\r\nc"), Err(Err::Error(("ab\r\nc", ErrorKind::CrLf))));
/// assert_eq!(line_ending::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::line_ending`][crate::character::line_ending] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::line_ending` with input wrapped in `nom8::input::Streaming`"
)]
pub fn line_ending<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
  T: InputIter + InputLength,
  T: IntoOutput,
  T: Compare<&'static str>,
{
  match input.compare("\n") {
    CompareResult::Ok => Ok((input.slice(1..), input.slice(0..1))).into_output(),
    CompareResult::Incomplete => Err(Err::Incomplete(Needed::new(1))),
    CompareResult::Error => {
      match input.compare("\r\n") {
        //FIXME: is this the right index?
        CompareResult::Ok => Ok((input.slice(2..), input.slice(0..2))).into_output(),
        CompareResult::Incomplete => Err(Err::Incomplete(Needed::new(2))),
        CompareResult::Error => Err(Err::Error(E::from_error_kind(input, ErrorKind::CrLf))),
      }
    }
  }
}

/// Matches a newline character '\\n'.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::newline;
/// assert_eq!(newline::<_, (_, ErrorKind)>("\nc"), Ok(("c", '\n')));
/// assert_eq!(newline::<_, (_, ErrorKind)>("\r\nc"), Err(Err::Error(("\r\nc", ErrorKind::Char))));
/// assert_eq!(newline::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::newline`][crate::character::newline] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::newline` with input wrapped in `nom8::input::Streaming`"
)]
pub fn newline<I, Error: ParseError<I>>(input: I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + InputLength,
  <I as InputIter>::Item: AsChar,
{
  char('\n')(input)
}

/// Matches a tab character '\t'.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::tab;
/// assert_eq!(tab::<_, (_, ErrorKind)>("\tc"), Ok(("c", '\t')));
/// assert_eq!(tab::<_, (_, ErrorKind)>("\r\nc"), Err(Err::Error(("\r\nc", ErrorKind::Char))));
/// assert_eq!(tab::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::tab`][crate::character::tab] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::tab` with input wrapped in `nom8::input::Streaming`"
)]
pub fn tab<I, Error: ParseError<I>>(input: I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + InputLength,
  <I as InputIter>::Item: AsChar,
{
  char('\t')(input)
}

/// Matches one byte as a character. Note that the input type will
/// accept a `str`, but not a `&[u8]`, unlike many other nom parsers.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data.
/// # Example
///
/// ```
/// # use nom8::{character::streaming::anychar, Err, error::ErrorKind, IResult, Needed};
/// assert_eq!(anychar::<_, (_, ErrorKind)>("abc"), Ok(("bc",'a')));
/// assert_eq!(anychar::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::bytes::any`][crate::bytes::any] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::bytes::any` with input wrapped in `nom8::input::Streaming`"
)]
pub fn anychar<T, E: ParseError<T>>(input: T) -> IResult<T, char, E>
where
  T: InputIter + InputLength + Slice<RangeFrom<usize>>,
  <T as InputIter>::Item: AsChar,
{
  crate::bytes::streaming::any(input).map(|(i, c)| (i, c.as_char()))
}

/// Recognizes zero or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non alphabetic character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::alpha0;
/// assert_eq!(alpha0::<_, (_, ErrorKind)>("ab1c"), Ok(("1c", "ab")));
/// assert_eq!(alpha0::<_, (_, ErrorKind)>("1c"), Ok(("1c", "")));
/// assert_eq!(alpha0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::alpha0`][crate::character::alpha0] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::alpha0` with input wrapped in `nom8::input::Streaming`"
)]
pub fn alpha0<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position_streaming(|item| !item.is_alpha())
    .into_output()
}

/// Recognizes one or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non alphabetic character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::alpha1;
/// assert_eq!(alpha1::<_, (_, ErrorKind)>("aB1c"), Ok(("1c", "aB")));
/// assert_eq!(alpha1::<_, (_, ErrorKind)>("1c"), Err(Err::Error(("1c", ErrorKind::Alpha))));
/// assert_eq!(alpha1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::alpha1`][crate::character::alpha1] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::alpha1` with input wrapped in `nom8::input::Streaming`"
)]
pub fn alpha1<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position1_streaming(|item| !item.is_alpha(), ErrorKind::Alpha)
    .into_output()
}

/// Recognizes zero or more ASCII numerical characters: 0-9
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non digit character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::digit0;
/// assert_eq!(digit0::<_, (_, ErrorKind)>("21c"), Ok(("c", "21")));
/// assert_eq!(digit0::<_, (_, ErrorKind)>("a21c"), Ok(("a21c", "")));
/// assert_eq!(digit0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::digit0`][crate::character::digit0] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::digit0` with input wrapped in `nom8::input::Streaming`"
)]
pub fn digit0<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position_streaming(|item| !item.is_dec_digit())
    .into_output()
}

/// Recognizes one or more ASCII numerical characters: 0-9
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non digit character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::digit1;
/// assert_eq!(digit1::<_, (_, ErrorKind)>("21c"), Ok(("c", "21")));
/// assert_eq!(digit1::<_, (_, ErrorKind)>("c1"), Err(Err::Error(("c1", ErrorKind::Digit))));
/// assert_eq!(digit1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::digit1`][crate::character::digit1] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::digit1` with input wrapped in `nom8::input::Streaming`"
)]
pub fn digit1<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position1_streaming(|item| !item.is_dec_digit(), ErrorKind::Digit)
    .into_output()
}

/// Recognizes zero or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non hexadecimal digit character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::hex_digit0;
/// assert_eq!(hex_digit0::<_, (_, ErrorKind)>("21cZ"), Ok(("Z", "21c")));
/// assert_eq!(hex_digit0::<_, (_, ErrorKind)>("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(hex_digit0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::hex_digit0`][crate::character::hex_digit0] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::hex_digit0` with input wrapped in `nom8::input::Streaming`"
)]
pub fn hex_digit0<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position_streaming(|item| !item.is_hex_digit())
    .into_output()
}

/// Recognizes one or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non hexadecimal digit character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::hex_digit1;
/// assert_eq!(hex_digit1::<_, (_, ErrorKind)>("21cZ"), Ok(("Z", "21c")));
/// assert_eq!(hex_digit1::<_, (_, ErrorKind)>("H2"), Err(Err::Error(("H2", ErrorKind::HexDigit))));
/// assert_eq!(hex_digit1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::hex_digit1`][crate::character::hex_digit1] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::hex_digit1` with input wrapped in `nom8::input::Streaming`"
)]
pub fn hex_digit1<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position1_streaming(|item| !item.is_hex_digit(), ErrorKind::HexDigit)
    .into_output()
}

/// Recognizes zero or more octal characters: 0-7
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non octal digit character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::oct_digit0;
/// assert_eq!(oct_digit0::<_, (_, ErrorKind)>("21cZ"), Ok(("cZ", "21")));
/// assert_eq!(oct_digit0::<_, (_, ErrorKind)>("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(oct_digit0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::oct_digit0`][crate::character::oct_digit0] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::oct_digit0` with input wrapped in `nom8::input::Streaming`"
)]
pub fn oct_digit0<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position_streaming(|item| !item.is_oct_digit())
    .into_output()
}

/// Recognizes one or more octal characters: 0-7
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non octal digit character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::oct_digit1;
/// assert_eq!(oct_digit1::<_, (_, ErrorKind)>("21cZ"), Ok(("cZ", "21")));
/// assert_eq!(oct_digit1::<_, (_, ErrorKind)>("H2"), Err(Err::Error(("H2", ErrorKind::OctDigit))));
/// assert_eq!(oct_digit1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::oct_digit1`][crate::character::oct_digit1] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::oct_digit1` with input wrapped in `nom8::input::Streaming`"
)]
pub fn oct_digit1<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position1_streaming(|item| !item.is_oct_digit(), ErrorKind::OctDigit)
    .into_output()
}

/// Recognizes zero or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non alphanumerical character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::alphanumeric0;
/// assert_eq!(alphanumeric0::<_, (_, ErrorKind)>("21cZ%1"), Ok(("%1", "21cZ")));
/// assert_eq!(alphanumeric0::<_, (_, ErrorKind)>("&Z21c"), Ok(("&Z21c", "")));
/// assert_eq!(alphanumeric0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::alphanumeric0`][crate::character::alphanumeric0] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::alphanumeric0` with input wrapped in `nom8::input::Streaming`"
)]
pub fn alphanumeric0<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position_streaming(|item| !item.is_alphanum())
    .into_output()
}

/// Recognizes one or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non alphanumerical character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::alphanumeric1;
/// assert_eq!(alphanumeric1::<_, (_, ErrorKind)>("21cZ%1"), Ok(("%1", "21cZ")));
/// assert_eq!(alphanumeric1::<_, (_, ErrorKind)>("&H2"), Err(Err::Error(("&H2", ErrorKind::AlphaNumeric))));
/// assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::alphanumeric1`][crate::character::alphanumeric1] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::alphanumeric1` with input wrapped in `nom8::input::Streaming`"
)]
pub fn alphanumeric1<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input
    .split_at_position1_streaming(|item| !item.is_alphanum(), ErrorKind::AlphaNumeric)
    .into_output()
}

/// Recognizes zero or more spaces and tabs.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::space0;
/// assert_eq!(space0::<_, (_, ErrorKind)>(" \t21c"), Ok(("21c", " \t")));
/// assert_eq!(space0::<_, (_, ErrorKind)>("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(space0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::space0`][crate::character::space0] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::space0` with input wrapped in `nom8::input::Streaming`"
)]
pub fn space0<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
  input
    .split_at_position_streaming(|item| {
      let c = item.as_char();
      !(c == ' ' || c == '\t')
    })
    .into_output()
}
/// Recognizes one or more spaces and tabs.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::space1;
/// assert_eq!(space1::<_, (_, ErrorKind)>(" \t21c"), Ok(("21c", " \t")));
/// assert_eq!(space1::<_, (_, ErrorKind)>("H2"), Err(Err::Error(("H2", ErrorKind::Space))));
/// assert_eq!(space1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::space1`][crate::character::space1] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::space1` with input wrapped in `nom8::input::Streaming`"
)]
pub fn space1<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
  input
    .split_at_position1_streaming(
      |item| {
        let c = item.as_char();
        !(c == ' ' || c == '\t')
      },
      ErrorKind::Space,
    )
    .into_output()
}

/// Recognizes zero or more spaces, tabs, carriage returns and line feeds.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::multispace0;
/// assert_eq!(multispace0::<_, (_, ErrorKind)>(" \t\n\r21c"), Ok(("21c", " \t\n\r")));
/// assert_eq!(multispace0::<_, (_, ErrorKind)>("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(multispace0::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::multispace0`][crate::character::multispace0] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::multispace0` with input wrapped in `nom8::input::Streaming`"
)]
pub fn multispace0<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
  input
    .split_at_position_streaming(|item| {
      let c = item.as_char();
      !(c == ' ' || c == '\t' || c == '\r' || c == '\n')
    })
    .into_output()
}

/// Recognizes one or more spaces, tabs, carriage returns and line feeds.
///
/// *Streaming version*: Will return `Err(nom8::Err::Incomplete(_))` if there's not enough input data,
/// or if no terminating token is found (a non space character).
/// # Example
///
/// ```
/// # use nom8::{Err, error::ErrorKind, IResult, Needed};
/// # use nom8::character::streaming::multispace1;
/// assert_eq!(multispace1::<_, (_, ErrorKind)>(" \t\n\r21c"), Ok(("21c", " \t\n\r")));
/// assert_eq!(multispace1::<_, (_, ErrorKind)>("H2"), Err(Err::Error(("H2", ErrorKind::MultiSpace))));
/// assert_eq!(multispace1::<_, (_, ErrorKind)>(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
///
/// **WARNING:** Deprecated, replaced with [`nom8::character::multispace1`][crate::character::multispace1] with input wrapped in [`nom8::input::Streaming`][crate::input::Streaming]
#[deprecated(
  since = "8.0.0",
  note = "Replaced with `nom8::character::multispace1` with input wrapped in `nom8::input::Streaming`"
)]
pub fn multispace1<T, E: ParseError<T>>(input: T) -> IResult<T, <T as IntoOutput>::Output, E>
where
  T: InputTakeAtPosition,
  T: IntoOutput,
  <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
  input
    .split_at_position1_streaming(
      |item| {
        let c = item.as_char();
        !(c == ' ' || c == '\t' || c == '\r' || c == '\n')
      },
      ErrorKind::MultiSpace,
    )
    .into_output()
}

pub(crate) fn sign<T, E: ParseError<T>>(input: T) -> IResult<T, bool, E>
where
  T: Clone + InputTake + InputLength,
  T: IntoOutput,
  T: for<'a> Compare<&'a [u8]>,
{
  use crate::bytes::streaming::tag;
  use crate::combinator::value;

  let (i, opt_sign) = opt(alt((
    value(false, tag(&b"-"[..])),
    value(true, tag(&b"+"[..])),
  )))(input)?;
  let sign = opt_sign.unwrap_or(true);

  Ok((i, sign))
}

#[doc(hidden)]
macro_rules! ints {
    ($($t:tt)+) => {
        $(
        /// will parse a number in text form to a number
        ///
        /// *Complete version*: can parse until the end of input.
        pub fn $t<T, E: ParseError<T>>(input: T) -> IResult<T, $t, E>
            where
            T: InputIter + Slice<RangeFrom<usize>> + InputLength + InputTake + Clone,
            T: IntoOutput,
            <T as InputIter>::Item: AsChar,
            T: for <'a> Compare<&'a[u8]>,
            {
              let (i, sign) = sign(input.clone())?;

                if i.input_len() == 0 {
                    return Err(Err::Incomplete(Needed::new(1)));
                }

                let mut value: $t = 0;
                if sign {
                    for (pos, c) in i.iter_indices() {
                        match c.as_char().to_digit(10) {
                            None => {
                                if pos == 0 {
                                    return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit)));
                                } else {
                                    return Ok((i.slice(pos..), value));
                                }
                            },
                            Some(d) => match value.checked_mul(10).and_then(|v| v.checked_add(d as $t)) {
                                None => return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit))),
                                Some(v) => value = v,
                            }
                        }
                    }
                } else {
                    for (pos, c) in i.iter_indices() {
                        match c.as_char().to_digit(10) {
                            None => {
                                if pos == 0 {
                                    return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit)));
                                } else {
                                    return Ok((i.slice(pos..), value));
                                }
                            },
                            Some(d) => match value.checked_mul(10).and_then(|v| v.checked_sub(d as $t)) {
                                None => return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit))),
                                Some(v) => value = v,
                            }
                        }
                    }
                }

                Err(Err::Incomplete(Needed::new(1)))
            }
        )+
    }
}

ints! { i8 i16 i32 i64 i128 }

#[doc(hidden)]
macro_rules! uints {
    ($($t:tt)+) => {
        $(
        /// will parse a number in text form to a number
        ///
        /// *Complete version*: can parse until the end of input.
        pub fn $t<T, E: ParseError<T>>(input: T) -> IResult<T, $t, E>
            where
            T: InputIter + Slice<RangeFrom<usize>> + InputLength,
            T: IntoOutput,
            <T as InputIter>::Item: AsChar,
            {
                let i = input;

                if i.input_len() == 0 {
                    return Err(Err::Incomplete(Needed::new(1)));
                }

                let mut value: $t = 0;
                for (pos, c) in i.iter_indices() {
                    match c.as_char().to_digit(10) {
                        None => {
                            if pos == 0 {
                                return Err(Err::Error(E::from_error_kind(i, ErrorKind::Digit)));
                            } else {
                                return Ok((i.slice(pos..), value));
                            }
                        },
                        Some(d) => match value.checked_mul(10).and_then(|v| v.checked_add(d as $t)) {
                            None => return Err(Err::Error(E::from_error_kind(i, ErrorKind::Digit))),
                            Some(v) => value = v,
                        }
                    }
                }

                Err(Err::Incomplete(Needed::new(1)))
            }
        )+
    }
}

uints! { u8 u16 u32 u64 u128 }

#[cfg(test)]
mod tests {
  use super::*;
  use crate::error::ErrorKind;
  use crate::input::ParseTo;
  use crate::sequence::pair;
  use crate::{Err, IResult, Needed};
  use proptest::prelude::*;

  macro_rules! assert_parse(
    ($left: expr, $right: expr) => {
      let res: $crate::IResult<_, _, (_, ErrorKind)> = $left;
      assert_eq!(res, $right);
    };
  );

  #[test]
  fn anychar_str() {
    use super::anychar;
    assert_eq!(anychar::<_, (&str, ErrorKind)>("Ә"), Ok(("", 'Ә')));
  }

  #[test]
  fn character() {
    let a: &[u8] = b"abcd";
    let b: &[u8] = b"1234";
    let c: &[u8] = b"a123";
    let d: &[u8] = "azé12".as_bytes();
    let e: &[u8] = b" ";
    let f: &[u8] = b" ;";
    //assert_eq!(alpha1::<_, (_, ErrorKind)>(a), Err(Err::Incomplete(Needed::new(1))));
    assert_parse!(alpha1(a), Err(Err::Incomplete(Needed::new(1))));
    assert_eq!(alpha1(b), Err(Err::Error((b, ErrorKind::Alpha))));
    assert_eq!(alpha1::<_, (_, ErrorKind)>(c), Ok((&c[1..], &b"a"[..])));
    assert_eq!(
      alpha1::<_, (_, ErrorKind)>(d),
      Ok(("é12".as_bytes(), &b"az"[..]))
    );
    assert_eq!(digit1(a), Err(Err::Error((a, ErrorKind::Digit))));
    assert_eq!(
      digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(digit1(c), Err(Err::Error((c, ErrorKind::Digit))));
    assert_eq!(digit1(d), Err(Err::Error((d, ErrorKind::Digit))));
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(c),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(d),
      Ok(("zé12".as_bytes(), &b"a"[..]))
    );
    assert_eq!(hex_digit1(e), Err(Err::Error((e, ErrorKind::HexDigit))));
    assert_eq!(oct_digit1(a), Err(Err::Error((a, ErrorKind::OctDigit))));
    assert_eq!(
      oct_digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(oct_digit1(c), Err(Err::Error((c, ErrorKind::OctDigit))));
    assert_eq!(oct_digit1(d), Err(Err::Error((d, ErrorKind::OctDigit))));
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    //assert_eq!(fix_error!(b,(), alphanumeric1), Ok((empty, b)));
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(c),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(d),
      Ok(("é12".as_bytes(), &b"az"[..]))
    );
    assert_eq!(
      space1::<_, (_, ErrorKind)>(e),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(space1::<_, (_, ErrorKind)>(f), Ok((&b";"[..], &b" "[..])));
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn character_s() {
    let a = "abcd";
    let b = "1234";
    let c = "a123";
    let d = "azé12";
    let e = " ";
    assert_eq!(
      alpha1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(alpha1(b), Err(Err::Error((b, ErrorKind::Alpha))));
    assert_eq!(alpha1::<_, (_, ErrorKind)>(c), Ok((&c[1..], &"a"[..])));
    assert_eq!(alpha1::<_, (_, ErrorKind)>(d), Ok(("é12", &"az"[..])));
    assert_eq!(digit1(a), Err(Err::Error((a, ErrorKind::Digit))));
    assert_eq!(
      digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(digit1(c), Err(Err::Error((c, ErrorKind::Digit))));
    assert_eq!(digit1(d), Err(Err::Error((d, ErrorKind::Digit))));
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(
      hex_digit1::<_, (_, ErrorKind)>(c),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(hex_digit1::<_, (_, ErrorKind)>(d), Ok(("zé12", &"a"[..])));
    assert_eq!(hex_digit1(e), Err(Err::Error((e, ErrorKind::HexDigit))));
    assert_eq!(oct_digit1(a), Err(Err::Error((a, ErrorKind::OctDigit))));
    assert_eq!(
      oct_digit1::<_, (_, ErrorKind)>(b),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(oct_digit1(c), Err(Err::Error((c, ErrorKind::OctDigit))));
    assert_eq!(oct_digit1(d), Err(Err::Error((d, ErrorKind::OctDigit))));
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(a),
      Err(Err::Incomplete(Needed::new(1)))
    );
    //assert_eq!(fix_error!(b,(), alphanumeric1), Ok((empty, b)));
    assert_eq!(
      alphanumeric1::<_, (_, ErrorKind)>(c),
      Err(Err::Incomplete(Needed::new(1)))
    );
    assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(d), Ok(("é12", "az")));
    assert_eq!(
      space1::<_, (_, ErrorKind)>(e),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  use crate::input::Offset;
  #[test]
  fn offset() {
    let a = &b"abcd;"[..];
    let b = &b"1234;"[..];
    let c = &b"a123;"[..];
    let d = &b" \t;"[..];
    let e = &b" \t\r\n;"[..];
    let f = &b"123abcDEF;"[..];

    match alpha1::<_, (_, ErrorKind)>(a) {
      Ok((i, _)) => {
        assert_eq!(a.offset(i) + i.len(), a.len());
      }
      _ => panic!("wrong return type in offset test for alpha"),
    }
    match digit1::<_, (_, ErrorKind)>(b) {
      Ok((i, _)) => {
        assert_eq!(b.offset(i) + i.len(), b.len());
      }
      _ => panic!("wrong return type in offset test for digit"),
    }
    match alphanumeric1::<_, (_, ErrorKind)>(c) {
      Ok((i, _)) => {
        assert_eq!(c.offset(i) + i.len(), c.len());
      }
      _ => panic!("wrong return type in offset test for alphanumeric"),
    }
    match space1::<_, (_, ErrorKind)>(d) {
      Ok((i, _)) => {
        assert_eq!(d.offset(i) + i.len(), d.len());
      }
      _ => panic!("wrong return type in offset test for space"),
    }
    match multispace1::<_, (_, ErrorKind)>(e) {
      Ok((i, _)) => {
        assert_eq!(e.offset(i) + i.len(), e.len());
      }
      _ => panic!("wrong return type in offset test for multispace"),
    }
    match hex_digit1::<_, (_, ErrorKind)>(f) {
      Ok((i, _)) => {
        assert_eq!(f.offset(i) + i.len(), f.len());
      }
      _ => panic!("wrong return type in offset test for hex_digit"),
    }
    match oct_digit1::<_, (_, ErrorKind)>(f) {
      Ok((i, _)) => {
        assert_eq!(f.offset(i) + i.len(), f.len());
      }
      _ => panic!("wrong return type in offset test for oct_digit"),
    }
  }

  #[test]
  fn is_not_line_ending_bytes() {
    let a: &[u8] = b"ab12cd\nefgh";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(a),
      Ok((&b"\nefgh"[..], &b"ab12cd"[..]))
    );

    let b: &[u8] = b"ab12cd\nefgh\nijkl";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(b),
      Ok((&b"\nefgh\nijkl"[..], &b"ab12cd"[..]))
    );

    let c: &[u8] = b"ab12cd\r\nefgh\nijkl";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(c),
      Ok((&b"\r\nefgh\nijkl"[..], &b"ab12cd"[..]))
    );

    let d: &[u8] = b"ab12cd";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(d),
      Err(Err::Incomplete(Needed::Unknown))
    );
  }

  #[test]
  fn is_not_line_ending_str() {
    /*
    let a: &str = "ab12cd\nefgh";
    assert_eq!(not_line_ending(a), Ok((&"\nefgh"[..], &"ab12cd"[..])));

    let b: &str = "ab12cd\nefgh\nijkl";
    assert_eq!(not_line_ending(b), Ok((&"\nefgh\nijkl"[..], &"ab12cd"[..])));

    let c: &str = "ab12cd\r\nefgh\nijkl";
    assert_eq!(not_line_ending(c), Ok((&"\r\nefgh\nijkl"[..], &"ab12cd"[..])));

    let d = "βèƒôřè\nÂßÇáƒƭèř";
    assert_eq!(not_line_ending(d), Ok((&"\nÂßÇáƒƭèř"[..], &"βèƒôřè"[..])));

    let e = "βèƒôřè\r\nÂßÇáƒƭèř";
    assert_eq!(not_line_ending(e), Ok((&"\r\nÂßÇáƒƭèř"[..], &"βèƒôřè"[..])));
    */

    let f = "βèƒôřè\rÂßÇáƒƭèř";
    assert_eq!(not_line_ending(f), Err(Err::Error((f, ErrorKind::Tag))));

    let g2: &str = "ab12cd";
    assert_eq!(
      not_line_ending::<_, (_, ErrorKind)>(g2),
      Err(Err::Incomplete(Needed::Unknown))
    );
  }

  #[test]
  fn hex_digit_test() {
    let i = &b"0123456789abcdefABCDEF;"[..];
    assert_parse!(hex_digit1(i), Ok((&b";"[..], &i[..i.len() - 1])));

    let i = &b"g"[..];
    assert_parse!(
      hex_digit1(i),
      Err(Err::Error(error_position!(i, ErrorKind::HexDigit)))
    );

    let i = &b"G"[..];
    assert_parse!(
      hex_digit1(i),
      Err(Err::Error(error_position!(i, ErrorKind::HexDigit)))
    );

    assert!(AsChar::is_hex_digit(b'0'));
    assert!(AsChar::is_hex_digit(b'9'));
    assert!(AsChar::is_hex_digit(b'a'));
    assert!(AsChar::is_hex_digit(b'f'));
    assert!(AsChar::is_hex_digit(b'A'));
    assert!(AsChar::is_hex_digit(b'F'));
    assert!(!AsChar::is_hex_digit(b'g'));
    assert!(!AsChar::is_hex_digit(b'G'));
    assert!(!AsChar::is_hex_digit(b'/'));
    assert!(!AsChar::is_hex_digit(b':'));
    assert!(!AsChar::is_hex_digit(b'@'));
    assert!(!AsChar::is_hex_digit(b'\x60'));
  }

  #[test]
  fn oct_digit_test() {
    let i = &b"01234567;"[..];
    assert_parse!(oct_digit1(i), Ok((&b";"[..], &i[..i.len() - 1])));

    let i = &b"8"[..];
    assert_parse!(
      oct_digit1(i),
      Err(Err::Error(error_position!(i, ErrorKind::OctDigit)))
    );

    assert!(AsChar::is_oct_digit(b'0'));
    assert!(AsChar::is_oct_digit(b'7'));
    assert!(!AsChar::is_oct_digit(b'8'));
    assert!(!AsChar::is_oct_digit(b'9'));
    assert!(!AsChar::is_oct_digit(b'a'));
    assert!(!AsChar::is_oct_digit(b'A'));
    assert!(!AsChar::is_oct_digit(b'/'));
    assert!(!AsChar::is_oct_digit(b':'));
    assert!(!AsChar::is_oct_digit(b'@'));
    assert!(!AsChar::is_oct_digit(b'\x60'));
  }

  #[test]
  fn full_line_windows() {
    fn take_full_line(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
      pair(not_line_ending, line_ending)(i)
    }
    let input = b"abc\r\n";
    let output = take_full_line(input);
    assert_eq!(output, Ok((&b""[..], (&b"abc"[..], &b"\r\n"[..]))));
  }

  #[test]
  fn full_line_unix() {
    fn take_full_line(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
      pair(not_line_ending, line_ending)(i)
    }
    let input = b"abc\n";
    let output = take_full_line(input);
    assert_eq!(output, Ok((&b""[..], (&b"abc"[..], &b"\n"[..]))));
  }

  #[test]
  fn check_windows_lineending() {
    let input = b"\r\n";
    let output = line_ending(&input[..]);
    assert_parse!(output, Ok((&b""[..], &b"\r\n"[..])));
  }

  #[test]
  fn check_unix_lineending() {
    let input = b"\n";
    let output = line_ending(&input[..]);
    assert_parse!(output, Ok((&b""[..], &b"\n"[..])));
  }

  #[test]
  fn cr_lf() {
    assert_parse!(crlf(&b"\r\na"[..]), Ok((&b"a"[..], &b"\r\n"[..])));
    assert_parse!(crlf(&b"\r"[..]), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      crlf(&b"\ra"[..]),
      Err(Err::Error(error_position!(&b"\ra"[..], ErrorKind::CrLf)))
    );

    assert_parse!(crlf("\r\na"), Ok(("a", "\r\n")));
    assert_parse!(crlf("\r"), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      crlf("\ra"),
      Err(Err::Error(error_position!("\ra", ErrorKind::CrLf)))
    );
  }

  #[test]
  fn end_of_line() {
    assert_parse!(line_ending(&b"\na"[..]), Ok((&b"a"[..], &b"\n"[..])));
    assert_parse!(line_ending(&b"\r\na"[..]), Ok((&b"a"[..], &b"\r\n"[..])));
    assert_parse!(
      line_ending(&b"\r"[..]),
      Err(Err::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      line_ending(&b"\ra"[..]),
      Err(Err::Error(error_position!(&b"\ra"[..], ErrorKind::CrLf)))
    );

    assert_parse!(line_ending("\na"), Ok(("a", "\n")));
    assert_parse!(line_ending("\r\na"), Ok(("a", "\r\n")));
    assert_parse!(line_ending("\r"), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      line_ending("\ra"),
      Err(Err::Error(error_position!("\ra", ErrorKind::CrLf)))
    );
  }

  fn digit_to_i16(input: &str) -> IResult<&str, i16> {
    let i = input;
    let (i, opt_sign) = opt(alt((char('+'), char('-'))))(i)?;
    let sign = match opt_sign {
      Some('+') => true,
      Some('-') => false,
      _ => true,
    };

    let (i, s) = match digit1::<_, crate::error::Error<_>>(i) {
      Ok((i, s)) => (i, s),
      Err(Err::Incomplete(i)) => return Err(Err::Incomplete(i)),
      Err(_) => {
        return Err(Err::Error(crate::error::Error::from_error_kind(
          input,
          ErrorKind::Digit,
        )))
      }
    };
    match s.parse_to() {
      Some(n) => {
        if sign {
          Ok((i, n))
        } else {
          Ok((i, -n))
        }
      }
      None => Err(Err::Error(crate::error::Error::from_error_kind(
        i,
        ErrorKind::Digit,
      ))),
    }
  }

  fn digit_to_u32(i: &str) -> IResult<&str, u32> {
    let (i, s) = digit1(i)?;
    match s.parse_to() {
      Some(n) => Ok((i, n)),
      None => Err(Err::Error(crate::error::Error::from_error_kind(
        i,
        ErrorKind::Digit,
      ))),
    }
  }

  #[test]
  fn one_of_test() {
    fn f(i: &[u8]) -> IResult<&[u8], char> {
      one_of("ab")(i)
    }

    let a = &b"abcd"[..];
    assert_eq!(f(a), Ok((&b"bcd"[..], 'a')));

    let b = &b"cde"[..];
    assert_eq!(f(b), Err(Err::Error(error_position!(b, ErrorKind::OneOf))));

    fn utf8(i: &str) -> IResult<&str, char> {
      one_of("+\u{FF0B}")(i)
    }

    assert!(utf8("+").is_ok());
    assert!(utf8("\u{FF0B}").is_ok());
  }

  #[test]
  fn none_of_test() {
    fn f(i: &[u8]) -> IResult<&[u8], char> {
      none_of("ab")(i)
    }

    let a = &b"abcd"[..];
    assert_eq!(f(a), Err(Err::Error(error_position!(a, ErrorKind::NoneOf))));

    let b = &b"cde"[..];
    assert_eq!(f(b), Ok((&b"de"[..], 'c')));
  }

  #[test]
  fn char_byteslice() {
    fn f(i: &[u8]) -> IResult<&[u8], char> {
      char('c')(i)
    }

    let a = &b"abcd"[..];
    assert_eq!(f(a), Err(Err::Error(error_position!(a, ErrorKind::Char))));

    let b = &b"cde"[..];
    assert_eq!(f(b), Ok((&b"de"[..], 'c')));
  }

  #[test]
  fn char_str() {
    fn f(i: &str) -> IResult<&str, char> {
      char('c')(i)
    }

    let a = &"abcd"[..];
    assert_eq!(f(a), Err(Err::Error(error_position!(a, ErrorKind::Char))));

    let b = &"cde"[..];
    assert_eq!(f(b), Ok((&"de"[..], 'c')));
  }

  proptest! {
    #[test]
    fn ints(s in "\\PC*") {
        let res1 = digit_to_i16(&s);
        let res2 = i16(s.as_str());
        assert_eq!(res1, res2);
    }

    #[test]
    fn uints(s in "\\PC*") {
        let res1 = digit_to_u32(&s);
        let res2 = u32(s.as_str());
        assert_eq!(res1, res2);
    }
  }
}
