
#[allow(non_upper_case_globals)]
pub const _where: &'static str = "WHERE";

#[macro_export]
macro_rules! _where {
    ( $( $clause:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($clause);)*
        format!("WHERE {}", tmp_vec.join(" AND "))
    }};
}

#[allow(non_upper_case_globals)]
pub const and: &'static str = "AND";

#[macro_export]
macro_rules! and {
    ( $( $arg:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($arg.to_string());)*
        format!("({})", tmp_vec.join(" AND "))
    }};
}

#[allow(non_upper_case_globals)]
pub const or: &'static str = "OR";

#[macro_export]
macro_rules! or {
    ( $( $arg:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($arg.to_string());)*
        format!("({})", tmp_vec.join(" OR "))
    }};
}

#[allow(non_upper_case_globals)]
pub const _in: &'static str = "IN";

#[macro_export]
macro_rules! _in {
    ( $( $arg:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($arg.to_string());)*
        format!("IN ({})", tmp_vec.join(", "))
    }};
}

#[allow(non_upper_case_globals)]
pub const not_in: &'static str = "NOT IN";

#[macro_export]
macro_rules! not_in {
    ( $( $arg:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($arg.to_string());)*
        format!("NOT IN ({})", tmp_vec.join(", "))
    }};
}

#[allow(non_upper_case_globals)]
pub const exists: &'static str = "EXISTS";

#[macro_export]
macro_rules! exists {
    ( $( $arg:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(
            tmp_vec.push($arg.to_string());
        )*
        format!("EXISTS ({})", tmp_vec.join(", "))
    }};
}

#[allow(non_upper_case_globals)]
pub const any: &'static str = "ANY";

#[macro_export]
macro_rules! any {
    ( $( $arg:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(
            tmp_vec.push($arg.to_string());
        )*
        format!("ANY ({})", tmp_vec.join(", "))
    }};
}

#[allow(non_upper_case_globals)]
pub const all: &'static str = "ALL";

#[macro_export]
macro_rules! all {
    ( $( $arg:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(
            tmp_vec.push($arg.to_string());
        )*
        format!("ALL ({})", tmp_vec.join(", "))
    }};
}

#[allow(non_upper_case_globals)]
pub const some: &'static str = "SOME";

#[macro_export]
macro_rules! some {
    ( $( $arg:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(
            tmp_vec.push($arg.to_string());
        )*
        format!("SOME ({})", tmp_vec.join(", "))
    }};
}

#[macro_export]
macro_rules! between {
    ( $column:expr, $left:expr, $right:expr ) => {{
        format!("{} BETWEEN {} AND {}", $column, $left, $right)
    }};
}

#[macro_export]
macro_rules! not_between {
    ( $column:expr, $left:expr, $right:expr ) => {{
        format!("{} NOT BETWEEN {} AND {}", $column, $left, $right)
    }};
}

#[macro_export]
macro_rules! is_null {
    ( $arg:expr ) => {{
        format!("{} IS NULL", $arg)
    }};
}

#[macro_export]
macro_rules! is_not_null {
    ( $arg:expr ) => {{
        format!("{} IS NOT NULL", $arg)
    }};
}

#[allow(non_upper_case_globals)]
pub const if_not_exists: &'static str = "IF NOT EXISTS";

#[macro_export]
macro_rules! like {
    ( $arg:expr ) => {
        format!("LIKE {}", $arg)
    };
}

#[macro_export]
macro_rules! not_like {
    ( $arg:expr ) => {
        format!("NOT LIKE {}", $arg)
    };
}

#[macro_export]
macro_rules! having {
    ( $condition:expr ) => {
        format!("HAVING {}", $condition)
    };
}

#[macro_export]
macro_rules! case {
    ( $( $arg:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($arg.to_string());)*
        format!("CASE {} END", tmp_vec.join(" "))
    }};
}

#[macro_export]
macro_rules! when_then {
    ( $condition:expr, $result:expr ) => {
        format!("WHEN {} THEN {}", $condtion, $result)
    };
}

#[macro_export]
macro_rules! insert_key_values {
    ( $( $key_values:expr ),* ) => {{
        let mut columns = Vec::new();
        let mut values = Vec::new();
        $(
            columns.push($key_values.0.to_string());
            values.push($key_values.1.to_string());
        )*
        format!(
            "({}) VALUES ({})",
            columns.join(", "),
            values.join(", ")
        )
    }};
}

#[macro_export]
macro_rules! key_value {
    ( $key:expr, $value:expr ) => {
        ($key, $value)
    };
}

#[macro_export]
macro_rules! values {
    ( $( $column:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($column.to_string());)*
        format!("VALUES ({})", tmp_vec.join(", "))
    }};
}