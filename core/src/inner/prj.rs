
use model::* ;
use def::* ;
use res::* ;
pub struct Project 
{
    name   : String,

}
impl Project 
{
    pub fn new(name : String) -> Project
    {
        Project {name,} 
    }
    pub fn load(data: StrMap ) ->Project
    {
        let name = data.must_get(&String::from("_name")) ;
        let prj  = Project::new(name.clone()) ;
        return prj ;
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
        fn res_info(&self,_context : &mut Context) ->BoolR 
        {
            Ok(())

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
