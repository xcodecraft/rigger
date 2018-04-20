
use model::* ;
use def::* ;
use res::* ;
use super::* ;
#[derive(Debug)]
pub struct Project 
{
    name   : String,
    resvec : ResVec,

}
impl Project 
{
    pub fn new(name : String) -> Project
    {
        Project {name,resvec: ResVec::new() } 
    }
    pub fn load(data: StrMap ) ->Project
    {
        let name = data.must_get(&String::from("_name")) ;
        let prj  = Project::new(name.clone()) ;
        return prj ;
    }

}

impl InnerContainer for Project {

    fn resvec_hold<'a>(&'a mut self) ->&'a mut  ResVec 
    {
        &mut self.resvec
    }
}

impl StartBehavior for Project
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
        format!("project : {}",self.name)

    }
}
impl StopBehavior for Project 
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
impl CallPlugin for Project 
{
    fn res_before(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())
    }

    fn res_after(&self,_context : &mut Context) ->BoolR 
    {
        trace!("Project::res_after") ;
        Ok(())

    }
}
