#[macro_export]
macro_rules! map {
    {} => ($crate::StrMap::new());
    // In this implementation, key/value pairs separated by commas.
    { $( $key:expr => $value:expr ),* } => {
         map!( $( $key => $value, )* )
    };

    { $( $key:expr => $value:expr, )* } => ({
        use $crate::StrMap;
        let mut map = StrMap::new();
        $(
            map.insert(String::from($key), String::from($value));
         )*
        map 
     })
}

#[macro_export]
macro_rules! rg_sh {
    ( $( $cmd:tt )* ) => {{
        info!($($cmd)*) ;
        use shells ;
        shells::execute_with("sh", &format!($( $cmd )*))
    }};
}
