
#[macro_export]
macro_rules! compose {
    ( $( $arg:expr ),* ) => {{
        join_args_with!(" " $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! join_args_with {
    ( $sep:expr, $( $arg:expr ),* ) => {{
        [$(format!("{}", $arg),)*].join($sep)
    }};
}

#[macro_export]
macro_rules! enclose {
    ( $arg:expr ) => {{
        format!("({})", $arg)
    }};
}

#[allow(non_upper_case_globals)]
pub const escape: &'static str = "ESCAPE";

#[macro_export]
macro_rules! escape {
    ( $arg:expr ) => {
        format!("'{}'", format!("{}", $arg).as_str().replace("'", "''"))
    };
}