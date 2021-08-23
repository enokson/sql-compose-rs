
#[macro_export]
macro_rules! join {
    ( $join_type:expr, $table_name:expr, $condition:expr ) => {{
        format!("{} JOIN {} ON {}", $join_type, $table_name, $condition)
    }};
}

#[macro_export]
macro_rules! inner {
    ( $table_name:expr, $condition:expr ) => {{
        join!("INNER", $table_name, $condition)
    }};
}

#[macro_export]
macro_rules! left {
    ( $table_name:expr, $condition:expr ) => {{
        join!("LEFT", $table_name, $condition)
    }};
}

#[macro_export]
macro_rules! right {
    ( $join_type:expr, $table_name:expr, $condition:expr ) => {{
        join!("RIGHT", $table_name, $condition)
    }};
}

#[macro_export]
macro_rules! full_outer {
    ( $join_type:expr, $table_name:expr, $condition:expr ) => {{
        join!("FULL OUTER", $table_name, $condition)
    }};
}

#[allow(non_upper_case_globals)]
pub const union_join: &'static str = "UNION";

#[allow(non_upper_case_globals)]
pub const union_join_all: &'static str = "UNION ALL";