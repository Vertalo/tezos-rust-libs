macro_rules! dispatch {
    ($match_parser: expr; $( $pat:pat $(if $pred:expr)? => $expr: expr ),+ $(,)? ) => {
        move |i|
        {
            let (i, initial) = $match_parser.parse(i)?;
            match initial {
                $(
                    $pat $(if $pred)? => $expr.parse(i),
                )*
            }
        }
    }
}
