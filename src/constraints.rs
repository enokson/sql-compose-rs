// CONSTRAINTS

#[allow(non_upper_case_globals)]
pub const not_null: &'static str = "NOT NULL";

#[allow(non_upper_case_globals)]
pub const auto_increment: &'static str = "AUTO_INCREMENT";

#[allow(non_upper_case_globals)]
pub const unique: &'static str = "UNIQUE";

#[allow(non_upper_case_globals)]
pub const constraint: &'static str = "CONSTRAINT";

#[macro_export]
macro_rules! constaint {
    ( $constraint:expr ) => {
        compose!(constraint, $constraint)
    };
}

#[allow(non_upper_case_globals)]
pub const check: &'static str = "CHECK";

#[macro_export]
macro_rules! check {
    ( $( $condition:expr ),* ) => {{
        compose!(check, enclose!(join_args_with(" AND " $(, $condition)*)))
    }};
}

#[allow(non_upper_case_globals)]
pub const default: &'static str = "DEFAULT";

#[macro_export]
macro_rules! default {
    ( $value:expr ) => {
        compose!(default, $value)
    };
}

#[allow(non_upper_case_globals)]
pub const primary_key: &'static str = "PRIMARY KEY";

#[macro_export]
macro_rules! primary_key {
    ( $key:expr ) => {
        data_type_fn!(primary_key, $key)
    };
}

#[macro_export]
macro_rules! foreign_key {
    ( $key:expr, $ref_table:expr, $ref_key:expr ) => {
        format!(
            "FOREIGN KEY ({}) REFERENCES {}({})",
            $key, $ref_table, $ref_key
        )
    };
}
