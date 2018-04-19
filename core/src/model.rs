use err ;
use def::* ;
use std::collections::HashMap ;
pub struct Context
{

}
impl Context 
{
    pub fn new() -> Context {
        Context{}
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
    fn info(&self  , context : &mut Context ) ->BoolR;
}
pub type ResBox = Box<Res> ;
pub type ResVec = Vec<Box<Res>> ;

pub enum  RgvType{
    Vars,
    Env,
    System,
    Project,
    //Modul,
    Res,
}

pub trait Parser
{
    fn  next(&self) -> Option<(RgvType,StrMap)> ;
}
pub type ParserBox = Box<Parser> ;

