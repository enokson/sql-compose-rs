#[macro_export]
macro_rules! data_type_fn {
    ( $data_type:expr, $value:expr ) => {
        format!("{}({})", $data_type, $value)
    };
}

#[allow(non_upper_case_globals)]
pub const char_type: &'static str = "CHAR";

#[macro_export]
macro_rules! char_type {
    ( $value:expr ) => {
        data_type_fn!("CHAR", $value)
    };
}

#[macro_export]
macro_rules! varchar {
    ( $value:expr ) => {
        data_type_fn!("VARCHAR", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const binary: &'static str = "BINARY";

#[macro_export]
macro_rules! binary {
    ( $value:expr ) => {
        data_type_fn!("BINARY", $value)
    };
}

#[macro_export]
macro_rules! var_binary {
    ( $value:expr ) => {
        data_type_fn!("VARBINARY", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const tiny_blob: &'static str = "TINYBLOB";

#[macro_export]
macro_rules! tiny_blob {
    ( $value:expr ) => {
        data_type_fn!("TINYBLOB", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const tiny_text: &'static str = "TINYTEXT";

#[macro_export]
macro_rules! tiny_text {
    ( $value:expr ) => {
        data_type_fn!("TINYTEXT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const text: &'static str = "TEXT";

#[macro_export]
macro_rules! text {
    ( $value:expr ) => {
        data_type_fn!("TEXT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const blob: &'static str = "BLOB";

#[macro_export]
macro_rules! blob {
    ( $value:expr ) => {
        data_type_fn!("BLOB", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const medium_text: &'static str = "MEDIUMTEXT";

#[macro_export]
macro_rules! medium_text {
    ( $value:expr ) => {
        data_type_fn!("MEDIUMTEXT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const medium_blob: &'static str = "MEDIUMBLOB";

#[macro_export]
macro_rules! medium_blob {
    ( $value:expr ) => {
        data_type_fn!("MEDIUMBLOB", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const long_text: &'static str = "LONGTEXT";

#[macro_export]
macro_rules! long_text {
    ( $value:expr ) => {
        data_type_fn!("LONGTEXT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const long_blob: &'static str = "LONGBLOB";

#[macro_export]
macro_rules! long_blob {
    ( $value:expr ) => {
        data_type_fn!("LONGBLOB", $value)
    };
}

#[macro_export]
macro_rules! enum_type {
    ( $( $arg:expr ),* ) => {{
        format!("ENUM({})", join_args_with!(", " $(, escape!($arg))*))
    }};
}

#[macro_export]
macro_rules! set_type {
    ( $( $arg:expr ),* ) => {{
        format!("SET({})", join_args_with!(", " $(, escape!($arg))*))
    }};
}

#[allow(non_upper_case_globals)]
pub const bit_type: &'static str = "BIT";

#[macro_export]
macro_rules! bit_type {
    ( $value:expr ) => {
        data_type_fn!("BIT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const tiny_int: &'static str = "TINYINT";

#[macro_export]
macro_rules! tiny_int {
    ( $value:expr ) => {
        data_type_fn!("TINYINT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const small_int: &'static str = "SMALLINT";

#[macro_export]
macro_rules! small_int {
    ( $value:expr ) => {
        data_type_fn!("SMALLINT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const medium_int: &'static str = "MEDIUMINT";

#[macro_export]
macro_rules! medium_int {
    ( $value:expr ) => {
        data_type_fn!("MEDIUMINT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const int: &'static str = "INT";

#[macro_export]
macro_rules! int {
    ( $value:expr ) => {
        data_type_fn!("INT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const integer: &'static str = "INTEGER";

#[macro_export]
macro_rules! integer {
    ( $value:expr ) => {
        data_type_fn!("INTEGER", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const big_int: &'static str = "BIGINT";

#[macro_export]
macro_rules! big_int {
    ( $value:expr ) => {
        data_type_fn!("BIGINT", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const float: &'static str = "FLOAT";

#[macro_export]
macro_rules! float {
    ( $size:expr ) => {
        data_type_fn!("FLOAT", $size)
    };
    ( $size:expr,$ d:expr ) => {
        data_type_fn!("FLOAT", format!("{}, {}", $size, $d))
    };
}

#[allow(non_upper_case_globals)]
pub const double: &'static str = "DOUBLE";

#[macro_export]
macro_rules! double {
    ( $size:expr ) => {
        data_type_fn!("DOUBLE", $size)
    };
    ( $size:expr,$ d:expr ) => {
        data_type_fn!("DOUBLE", format!("{}, {}", $size, $d))
    };
}

#[allow(non_upper_case_globals)]
pub const double_precision: &'static str = "DOUBLE PRECISION";

#[macro_export]
macro_rules! double_precision {
    ( $size:expr ) => {
        data_type_fn!("DOUBLE PRECISION", $size)
    };
    ( $size:expr,$ d:expr ) => {
        data_type_fn!("DOUBLE PRECISION", format!("{}, {}", $size, $d))
    };
}

#[allow(non_upper_case_globals)]
pub const decimal: &'static str = "DECIMAL";

#[macro_export]
macro_rules! decimal {
    ( $size:expr ) => {
        data_type_fn!("DECIMAL", $size)
    };
    ( $size:expr,$ d:expr ) => {
        data_type_fn!("DECIMAL", format!("{}, {}", $size, $d))
    };
}

#[allow(non_upper_case_globals)]
pub const dec: &'static str = "DEC";

#[macro_export]
macro_rules! dec {
    ( $size:expr ) => {
        data_type_fn!("DEC", $size)
    };
    ( $size:expr,$ d:expr ) => {
        data_type_fn!("DEC", format!("{}, {}", $size, $d))
    };
}

#[allow(non_upper_case_globals)]
pub const date: &'static str = "DATE";

#[allow(non_upper_case_globals)]
pub const date_time: &'static str = "DATETIME";

#[macro_export]
macro_rules! date_time {
    ( $value:expr ) => {
        data_type_fn!("DATETIME", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const timestamp: &'static str = "TIMESTAMP";

#[macro_export]
macro_rules! timestamp {
    ( $value:expr ) => {
        data_type_fn!("TIMESTAMP", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const time: &'static str = "TIME";

#[macro_export]
macro_rules! time {
    ( $value:expr ) => {
        data_type_fn!("TIME", $value)
    };
}

#[allow(non_upper_case_globals)]
pub const year: &'static str = "YEAR";
