use std::collections::HashMap ;
use std::convert::{From } ;
pub type StrMap =  HashMap<String,String> ;


#[derive(Debug,Clone)]
pub struct UString(String) ;
impl UString
{
    pub fn to_string(self) -> String
    {
        self.0
    }
}

impl From<String> for UString{
    fn from(val: String) -> Self {
        UString(val)
    }
}
impl <'a> From<&'a str > for UString {
    fn from(val: &'a str) -> Self {
        UString(String::from(val)) 
    }
}
impl <'a> From<&'a String> for UString {
    fn from(val: &'a String) -> Self {
        UString(val.clone()) 
    }
}

#[cfg(test)]
mod tests
{
    use super::* ;

    fn use_ustr<T>(val :T)
        where UString : From<T> 
    {
        let uval = UString::from(val);
        let sval : String = uval.0 ;
        debug!("{}",sval) ;

    }
    #[test]
    pub fn ustr_test()
    {
        
        use_ustr(format!("china"));
        use_ustr(&format!("china"));
        use_ustr("china");

    }

}
