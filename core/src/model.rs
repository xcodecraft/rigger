use err ; 
use def::* ;
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
impl <'a> From<&'a str > for CtxValue{
    fn from(val: &'a str) -> Self {
        CtxValue::Str(String::from(val)) 
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
    pub fn set<E,T>(&mut self,k : E, val : T)
        where String: std::convert::From<E> ,
        CtxValue: std::convert::From<T> 
    {
        let key = String::from(k);
        let obj = CtxValue::from(val);
        self.maps.insert(key,obj ) ;
        
    }
    pub fn get<E,T>(& self, k :E)->Option<T>
        where String: std::convert::From<E> ,
        CtxValue: std::convert::Into<T>
    {

        let key  = String::from(k) ;
        self.maps.get(&key).and_then( |val| Some(val.clone().into()) )
    }
    pub fn must_get<E,T>(& self, k :E)->T
        where String: std::convert::From<E> ,
        CtxValue: std::convert::Into<T>
    {
        let key  = String::from(k) ;
        let fund = self.maps.get(&key) ;
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
    fn clean(&self , context : &mut Context ) ->BoolR; 
    fn info(&self) ->String  ; fn name(&self) ->String  ; 
} 
pub type ResBox = Box<Res> ; 
pub type ResVec = Vec<Box<Res>> ;

impl std::fmt::Debug for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "info :{}", self.info())
    }

}
pub trait  Cmd
{
    fn execute(&self,res: ResBox, context : &mut Context);
}



#[cfg(test)]
mod tests
{
    use super::* ;

    #[test]
    fn context_use()
    {

        let mut ctx = Context::new();
        ctx.set("src",format!("hello src"));
        ctx.set(format!("dst"),"hello dst");
        let src : String = ctx.must_get("src") ;
        let dst : String = ctx.must_get("dst") ;
        assert_eq!(src,String::from("hello src")) ;
        assert_eq!(dst,String::from("hello dst")) ;
    }
}
