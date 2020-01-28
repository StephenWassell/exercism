#[macro_export]
macro_rules! hashmap {
    (
        $(
            $key:expr => $value:expr
        ),*
        $(,)? // to avoid an error on test_trailing_comma
    ) => {
        {
            let mut hm = std::collections::HashMap::new();
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    }
}
