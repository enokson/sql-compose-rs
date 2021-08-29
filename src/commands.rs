#[allow(non_upper_case_globals)]
pub const database: &'static str = "DATABASE";

#[macro_export]
macro_rules! database {
    ( $( $arg:expr ),* ) => {
        compose!(database $(, $arg)*)
    };
}

#[allow(non_upper_case_globals)]
pub const table: &'static str = "TABLE";

#[macro_export]
macro_rules! table {
    ( $( $arg:expr ),* ) => {
        compose!(table $(, $arg)*)
    };
}

#[allow(non_upper_case_globals)]
pub const column: &'static str = "COLUMN";

#[allow(non_upper_case_globals)]
pub const index: &'static str = "INDEX";

#[macro_export]
macro_rules! index {
    ( $( $arg:expr ),* ) => {
        compose!(index $(, $arg )*)
    };
}

#[allow(non_upper_case_globals)]
pub const view: &'static str = "VIEW";

#[macro_export]
macro_rules! view {
    ( $( $arg:expr ),* ) => {
        compose!(view $(, $arg )*)
    };
}

#[allow(non_upper_case_globals)]
pub const create: &'static str = "CREATE";

#[macro_export]
macro_rules! create {
    ( $( $arg:expr ),* ) => {
        compose!(create $(, $arg)*)
    };
}

#[macro_export]
macro_rules! create_database {
    ( $( $arg:expr ),* ) => {{
        create!(database $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! create_table {
    ( $( $arg:expr ),* ) => {{
        create!(table $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! create_index {
    ( $( $arg:expr ),* ) => {{
        create!(index $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! create_view {
    ( $( $arg:expr ),* ) => {{
        create!(view $(, $arg)*)
    }};
}

#[allow(non_upper_case_globals)]
pub const drop: &'static str = "DROP";

#[macro_export]
macro_rules! drop {
    ( $( $arg:expr ),* ) => {{
        compose!(drop $(, $arg )*)
    }};
}

#[macro_export]
macro_rules! drop_database {
    ( $( $arg:expr ),* ) => {{
        drop!(database $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! drop_table {
    ( $( $arg:expr ),* ) => {{
        drop!(table $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! drop_index {
    ( $( $arg:expr ),* ) => {{
        drop!(index $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! drop_view {
    ( $( $arg:expr ),* ) => {{
        drop!(view $(, $arg)*)
    }};
}

#[allow(non_upper_case_globals)]
pub const alter: &'static str = "ALTER";

#[macro_export]
macro_rules! alter {
    ( $( $arg:expr ),* ) => {{
        compose!(alter $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! alter_table {
    ( $( $arg:expr ),* ) => {{
        alter!(table $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! alter_column {
    ( $( $arg:expr ),* ) => {{
        alter!(column $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! select {
    ( $( $column:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($column.to_string());)*
        format!("SELECT {}", tmp_vec.join(", "))
    }};
}

#[macro_export]
macro_rules! select_distinct {
    ( $( $column:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($column.to_string());)*
        format!("SELECT DISTINCT {}", tmp_vec.join(", "))
    }};
}

#[macro_export]
macro_rules! insert_into {
    ( $table:expr ) => {
        format!("INSERT INTO {}", $table)
    };
}

#[allow(non_upper_case_globals)]
pub const delete: &'static str = "DELETE";

#[macro_export]
macro_rules! delete {
    ( $( $arg:expr ),* ) => {
        compose!(delete $(, $arg)*)
    };
}

#[macro_export]
macro_rules! delete_from {
    ( $table:expr ) => {
        format!("DELETE FROM {}", $table)
    };
}

#[macro_export]
macro_rules! update {
    ( $table:expr ) => {
        format!("UPDATE {}", $table)
    };
}

#[allow(non_upper_case_globals)]
pub const set: &'static str = "SET";

#[macro_export]
macro_rules! set {
    ( $( $column:expr ),* ) => {{
        let mut tmp_vec = Vec::new();
        $(tmp_vec.push($column.to_string());)*
        format!("SET {}", tmp_vec.join(", "))
    }};
}