use crate::*;

#[macro_export]
macro_rules! get_single {
    ($q:expr) => {
        match $q.single() {
            Ok(m) => m,
            _ => return,
        }
    };
}


#[macro_export]
macro_rules! get_single_mut {
    ($q:expr) => {
        match $q.single_mut() {
            Ok(m) => m,
            _ => return,
        }
    };
}
