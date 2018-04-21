
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


impl SellDesp for Project
{
    fn res_info(&self) -> String
    {
        format!("project: {}",self.name)

    }
    fn res_name(&self) -> String
    {
        format!("project: {}",self.name)

    }
    fn res_allow(&self,_context : &mut Context) ->BoolR 
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
