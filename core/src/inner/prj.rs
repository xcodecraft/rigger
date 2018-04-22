
inner_use!();
#[derive(Debug)]
pub struct Project 
{
    name   : String,
    resvec : RefCell<ResVec>,
}
impl Project 
{
    pub fn new(name : String) -> Project
    {
        Project {name,resvec: RefCell::new(ResVec::new()) } 
    }
    pub fn load(data: StrMap ) ->Project
    {
        let name = data.must_get(&String::from("_name")) ;
        let prj  = Project::new(name.clone()) ;
        return prj ;
    }

}


impl ResDesp for Project
{
    fn res_info(&self) -> String
    {
        format!("project: {}",self.name)
    }
    fn res_name(&self) -> String
    {
        format!("Project::{}",self.name)
    }
    fn res_allow(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
}

impl InvokeHook for Project 
{
    fn res_before(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())
    }

    fn res_after(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
}
impl InnerContainer for Project {

    fn resvec_hold(&self) ->RefMut<ResVec>
    {
        self.resvec.borrow_mut()
    }
}
