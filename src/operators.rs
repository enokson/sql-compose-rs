// comparators

#[macro_export]
macro_rules! compare {
    ( $left:expr, $operator:expr, $right:expr ) => {{
        compose!($left, $operator, $right)
    }};
}

#[allow(non_upper_case_globals)]
pub const eq: &'static str = "=";

#[macro_export]
macro_rules! eq {
    ( $left:expr, $right:expr ) => {{
        compare!($left, "=", $right)
    }};
}

#[allow(non_upper_case_globals)]
pub const ne: &'static str = "<>";

#[macro_export]
macro_rules! ne {
    ( $left:expr, $right:expr ) => {{
        compare!($left, "<>", $right)
    }};
}

#[allow(non_upper_case_globals)]
pub const gt: &'static str = ">";

#[macro_export]
macro_rules! gt {
    ( $left:expr, $right:expr ) => {{
        compare!($left, gt, $right)
    }};
}

#[allow(non_upper_case_globals)]
pub const gte: &'static str = ">=";

#[macro_export]
macro_rules! gte {
    ( $left:expr, $right:expr ) => {{
        compare!($left, gte, $right)
    }};
}

#[allow(non_upper_case_globals)]
pub const lt: &'static str = "<";

#[macro_export]
macro_rules! lt {
    ( $left:expr, $right:expr ) => {{
        compare!($left, lt, $right)
    }};
}

#[allow(non_upper_case_globals)]
pub const lte: &'static str = "<=";

#[macro_export]
macro_rules! lte {
    ( $left:expr, $right:expr ) => {{
        compare!($left, lte, $right)
    }};
}
