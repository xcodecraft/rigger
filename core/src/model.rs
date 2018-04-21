use err ;
use def::* ;
use std ;
use std::collections::HashMap ;
#[derive(Debug,Clone)]
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
    fn info(&self) ->String  ;
}
pub type ResBox = Box<Res> ;
pub type ResVec = Vec<Box<Res>> ;

impl std::fmt::Debug for Res
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "info :{}", self.info())
    }

}


