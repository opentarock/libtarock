#![macro_escape]

#[macro_export]
macro_rules! set(
    ($($x:expr),*) => ({
        use std::collections::HashSet;
        let mut set = HashSet::new();
        $(set.insert($x);)*
        set
    });
)
