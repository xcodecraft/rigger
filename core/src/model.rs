use err ; use def::* ;
use std ;
use std::convert::{From ,Into} ;
use std::collections::HashMap ;
#[derive(Debug,Clone)]
pub enum  CtxValue
{
    Str(String),
    f64(f64),
}

impl From<String> for CtxValue{
    fn from(val: String) -> Self {
        CtxValue::Str(val) 
    }
}
impl Into<String> for CtxValue {
    fn into(self) -> String {
        match self 
        {
            CtxValue::Str(val) => val ,
            _  => panic!("convert CtxValue to String Fail!"),
        }
    }
}

#[derive(Debug,Clone)]
pub struct Context
{
    maps : HashMap<String,CtxValue> ,
}
impl Context 
{
    pub fn new() -> Context {
        Context{ maps: HashMap::new() }
    }
    pub fn sset<T>(&mut self,key : String , val : T)
        where CtxValue: std::convert::From<T> 
    {
        let obj = CtxValue::from(val);
        self.maps.insert(key,obj ) ;
        
    }
    pub fn set<T>(&mut self,key : &str, val : T)
        where CtxValue: std::convert::From<T> 
    {
        let obj = CtxValue::from(val);
        self.maps.insert(String::from(key),obj ) ;
        
    }
    pub fn must_get<T>(& self, key :&str)->T
        where CtxValue: std::convert::Into<T>
    {
        self.must_sget(&String::from(key)) 
    }
    pub fn get<T>(& self, key :&str)->Option<T>
        where CtxValue: std::convert::Into<T>
    {
        self.sget::<T>(&String::from(key))
    }
    pub fn sget<T>(& self, key :&String)->Option<T>
        where CtxValue: std::convert::Into<T>
    {
        self.maps.get(key).and_then( |val| Some(val.clone().into()) )
    }

    pub fn must_sget<T>(& self, key :&String)->T
        where CtxValue: std::convert::Into<T>
    {
        let fund = self.maps.get(key) ;
        if let Some(val) = fund
        {
            return val.clone().into();

        }
        panic!("bad") ;
    }

    pub fn keep(&self) 
    {

    }
    pub fn rollback(&self)
    {
    }
}

pub trait CliCmd
{
    fn execute(&self) -> BoolR ;
}

pub trait Res
{
    fn allow(&self , context : &mut Context ) ->BoolR;
    fn conf(&self  , context : &mut Context ) ->BoolR ;
    fn start(&self , context : &mut Context ) ->BoolR;
    fn stop(&self  , context : &mut Context ) ->BoolR;
    fn check(&self , context : &mut Context ) ->BoolR;
    fn clean(&self , context : &mut Context ) ->BoolR; fn info(&self) ->String  ; fn name(&self) ->String  ; } pub type ResBox = Box<Res> ; pub type ResVec = Vec<Box<Res>> ; impl std::fmt::Debug for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "info :{}", self.info())
    }

}


#[cfg(test)]
mod tests
{
    use super::* ;

    #[test]
    fn context_use()
    {

        let mut ctx = Context::new();
        ctx.set("src",format!("hello"));
        let src : String = ctx.must_get("src") ;
    }
}
