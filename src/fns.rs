
#[macro_export]
macro_rules! min {
    ($arg:expr) => {
        data_type_fn!("MIN", $arg)
    };
}

#[macro_export]
macro_rules! max {
    ($arg:expr) => {
        data_type_fn!("MAX", $arg)
    };
}

#[macro_export]
macro_rules! count {
    ($arg:expr) => {
        data_type_fn!("COUNT", $arg)
    };
}

#[macro_export]
macro_rules! avg {
    ($arg:expr) => {
        data_type_fn!("AVG", $arg)
    };
}

#[macro_export]
macro_rules! sum {
    ($arg:expr) => {
        data_type_fn!("SUM", $arg)
    };
}
