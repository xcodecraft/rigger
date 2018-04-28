use rg_lib::* ;
use model::* ;
use res::* ;
use cmd::* ;
use def::* ;
use std::collections::HashMap ;
pub trait Loader <T> {
    fn load( data : &StrMap) -> T ;
    fn key() -> String ;
}
pub trait CmdLoader <T> {
    fn load() -> T ;
    fn key()  -> String ;
}

pub type ResCreator = fn(key :&String, data : &StrMap) ->  Option<Box<Res>>  ;
pub type CmdCreator = fn(key :&String ) ->  Option<Box<Cmd>>  ;

type ResCreatorMap = HashMap<String,ResCreator> ; 
type CmdCreatorMap = HashMap<String,CmdCreator> ; 

pub struct ResFatory
{
    creators : ResCreatorMap,
}
impl ResFatory
{
    pub fn new() -> ResFatory
    {
        ResFatory{ creators : ResCreatorMap::new() }
    }
    pub fn regist(&mut self ,key :String,creator : ResCreator )
    {
        self.creators.insert(key,creator) ;
    }
    pub fn create(&self,key :&String, data : &StrMap) -> Option<Box<Res>>
    {
        self.creators.get(key).and_then( | v | v(key,data) )
    }

}

pub fn res_createor_impl<T>(key :&String, data : &StrMap) -> Option<Box<Res>>
    where T: Loader<T> + ResDesp   + InvokeHook + InvokeStart + InvokeStop + 'static
{
    if *key == T::key()
    {
        let obj: ResBox  =  Box::new(T::load(data));
        return  Some(obj) ;
    }
    return None
}

pub fn regist_res_creator<T>(f : &mut ResFatory)
    where T: Loader<T> + ResDesp   + InvokeHook + InvokeStart + InvokeStop + 'static
{
     let fnobj : ResCreator = res_createor_impl::<T> ;
     f.regist(T::key(),fnobj) ;
}

pub struct CmdFatory
{
    creators : CmdCreatorMap,
}
impl CmdFatory
{
    pub fn new() -> CmdFatory
    {
        CmdFatory{ creators : CmdCreatorMap::new() }
    }
    pub fn regist(&mut self ,key :String,creator : CmdCreator )
    {
        self.creators.insert(key,creator) ;
    }
    pub fn create(&self,key :&String ) -> Option<Box<Cmd>>
    {
        self.creators.get(key).and_then( | v | v(key) )
    }
}

pub fn cmd_createor_impl<T>(key :&String ) -> Option<Box<Cmd>>
    where T: Cmd + CmdLoader<T> + CmdDesp   +  'static
{
    if *key == T::key()
    {
        let obj: CmdBox  =  Box::new(T::load());
        return  Some(obj) ;
    }
    return None
}

pub fn regist_cmd_creator<T>(f : &mut CmdFatory)
    where T: Cmd + CmdLoader<T> + CmdDesp   +  'static
{
     let fnobj : CmdCreator = cmd_createor_impl::<T> ;
     f.regist(T::key(),fnobj) ;
}
