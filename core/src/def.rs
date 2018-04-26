use std ;
use err ;
pub type Result<T > = std::result::Result<T, err::Error>;
pub type BoolR  = std::result::Result<(), err::Error>;
use rg_lib::StrMap ;

pub trait Mustable
{
    fn must_get<'a>(&'a self,key : &String)-> &'a String  ;
}
impl Mustable for StrMap {

    fn must_get<'a>(&'a self,key : &String)-> &'a String 
    {
        return self.get(key).expect(format!("not get {} value",key).as_str()) ;
    }

}

/*
#[macro_export]
macro_rules! map {
    // Empty object.
    {} => ($crate::def::StrMap::new());

    // Non-empty object, no trailing comma.
    //
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
        //StrMap(map) ;
     })
}

*/
