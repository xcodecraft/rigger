#[macro_export]
macro_rules! map {
    {} => ($crate::def::StrMap::new());
    // In this implementation, key/value pairs separated by commas.
    { $( $key:expr => $value:expr ),* } => {
         map!( $( $key => $value, )* )
    };

    { $( $key:expr => $value:expr, )* } => ({
        use $crate::def::StrMap;
        let mut map = StrMap::new();
        $(
            map.insert(String::from($key), String::from($value));
         )*
        map 
     })
}

