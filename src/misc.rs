#[allow(non_upper_case_globals)]
pub const on: &'static str = "ON";

#[macro_export]
macro_rules! on {
    ( $arg:expr ) => {
        compose!("ON", $arg)
    };
}

#[allow(non_upper_case_globals)]
pub const as_alias: &'static str = "AS";

#[macro_export]
macro_rules! as_alias {
    ( $arg:expr ) => {
        compose!("AS", $arg)
    };
}

#[allow(non_upper_case_globals)]
pub const add: &'static str = "ADD";

#[macro_export]
macro_rules! add {
    ( $arg:expr ) => {
        compose!("ADD", $arg)
    };
}

#[macro_export]
macro_rules! column {
    ( $( $arg:expr ),* ) => {{
        join_args_with!(" " $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! columns {
    ( $( $column:expr ),* ) => {{
        enclose!(join_args_with!(", " $(, $column)*))
    }};
}

#[macro_export]
macro_rules! as_column {
    ( $src:expr, $output:expr ) => {
        compose!($src, as_alias, $output)
    };
}

#[allow(non_upper_case_globals)]
pub const from: &'static str = "FROM";

#[macro_export]
macro_rules! from {
    ( $( $arg:expr ),* ) => {{
        compose!(from, join_args_with!(", " $(, $arg)*))
    }};
}

#[allow(non_upper_case_globals)]
pub const into: &'static str = "INTO";

#[macro_export]
macro_rules! into {
    ( $( $arg:expr ),* ) => {{
        compose!(into, join_args_with!(", " $(, $arg)*))
    }};
}

#[allow(non_upper_case_globals)]
pub const in_extern: &'static str = "IN";

#[macro_export]
macro_rules! in_extern {
    ( $( $arg:expr ),* ) => {{
        compose!(in_extern, join_args_with!(", " $(, $arg)*))
    }};
}

#[macro_export]
macro_rules! comment {
    ( $comment:expr ) => {{
        println!("/* {} */", $comment)
    }};
}

#[allow(non_upper_case_globals)]
pub const references: &'static str = "REFERENCES";

#[macro_export]
macro_rules! references {
    ( $( $arg:expr ),* ) => {{
        compose!(references, join_args_with!(", " $(, $arg)*))
    }};
}
