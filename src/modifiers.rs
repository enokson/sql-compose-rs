
#[allow(non_upper_case_globals)]
pub const order_by: &'static str = "ORDER BY";

#[macro_export]
macro_rules! order_by {
    ( $( $column:expr ),* ) => {{
        compose!(order_by, join_args_with!(", " $(, $column)*))
    }};
}

#[allow(non_upper_case_globals)]
pub const group_by: &'static str = "GROUP BY";

#[macro_export]
macro_rules! group_by {
    ( $( $column:expr ),* ) => {{
        compose!(group_by, join_args_with!(", " $(, $column)*))
    }};
}

#[allow(non_upper_case_globals)]
pub const limit: &'static str = "LIMIT";

#[macro_export]
macro_rules! limit {
    ( $limit:expr ) => {
        compose!(limit, $limit)
    };
}

#[allow(non_upper_case_globals)]
pub const asc: &'static str = "ASC";

#[macro_export]
macro_rules! asc {
    ( $column:expr ) => {
        compose!($column, asc)
    };
}

#[allow(non_upper_case_globals)]
pub const desc: &'static str = "DESC";

#[macro_export]
macro_rules! desc {
    ( $column:expr ) => {
        compose!($column, desc)
    };
}
