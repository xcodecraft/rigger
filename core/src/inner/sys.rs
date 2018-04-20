use model::* ;
use def::* ;
use res::* ;
use super::* ;
#[derive(Debug)]
pub struct System
{
    name   : String,
    limit  : String,
    resvec : ResVec,

}
impl System 
{
    pub fn new(name : String) -> System
    {
        System{name,  limit: String::new(),resvec :ResVec::new() }
    }
    pub fn load(data: StrMap ) ->System
    {
        let name = data.must_get(&String::from("_name")) ;
        let sys  = System::new(name.clone()) ;
        return sys ;

    }

}

impl InnerContainer for System {

    fn resvec_hold<'a>(&'a mut self) ->&'a mut  ResVec 
    {
        &mut self.resvec
    }
}

impl StartBehavior for System
{
    fn res_allow(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
    fn res_start(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
    fn res_conf(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
    fn res_info(&self) ->String
    {
        format!("system : {}", self.name)

    }
}
impl StopBehavior for System 
{
    fn res_stop(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
    fn res_clean(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
    fn res_check(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }

}
impl CallPlugin for System 
{
    fn res_before(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())
    }

    fn res_after(&self,_context : &mut Context) ->BoolR 
    {
        trace!("System::res_after") ;
        Ok(())

    }
}
