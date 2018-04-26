use rg_lib::* ;
use model::* ;
use res::* ;
use def::* ;
use std::collections::HashMap ;
pub trait Loader <T> {
    fn load( data : &StrMap) -> T ;
    fn key() -> String ;
}

pub type Creator = fn(key :&String, data : &StrMap) ->  Option<Box<Res>>  ;
type CreatorMap = HashMap<String,Creator> ; 

pub struct ResFatory
{
    creators : CreatorMap,
}
impl ResFatory
{
    pub fn new() -> ResFatory
    {
        ResFatory{ creators : CreatorMap::new() }
    }
    pub fn regist(&mut self ,key :String,creator : Creator )
    {
        self.creators.insert(key,creator) ;
    }
    pub fn create(&self,key :&String, data : &StrMap) -> Option<Box<Res>>
    {
        self.creators.get(key).and_then( | v | v(key,data) )
    }

}

pub fn createor_impl<T>(key :&String, data : &StrMap) -> Option<Box<Res>>
    where T: Loader<T> + ResDesp   + InvokeHook + InvokeStart + InvokeStop + 'static
{
    if *key == T::key()
    {
        let obj: ResBox  =  Box::new(T::load(data));
        return  Some(obj) ;
    }
    return None
}

pub fn regist_creator<T>(f : &mut ResFatory)
    where T: Loader<T> + ResDesp   + InvokeHook + InvokeStart + InvokeStop + 'static
{
     let fnobj : Creator = createor_impl::<T> ;
     f.regist(T::key(),fnobj) ;
}
