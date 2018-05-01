use std ;
use err ;
pub type Result<T > = std::result::Result<T, err::Error>;
pub type BoolR  = std::result::Result<(), err::Error>;
use rg_lib::StrMap ;

pub trait Mustable
{
    fn must_get<'a,T>(&'a self,key : &String)-> &'a String  ;
}
impl Mustable for StrMap {

    fn must_get<'a,T>(&'a self,key : &String)-> &'a String 
    {
        return self.get(key).expect(format!("{} not get {} value",T::key(),key).as_str()) ;
    }

}


