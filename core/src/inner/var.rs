
use model::* ;
use def::* ;
use res::* ;
#[derive(Debug)]
pub struct Vars
{
    kvmap  : StrMap  ,
}
impl Vars
{
    pub fn load( data : StrMap) -> Vars
    {
        Vars{ kvmap : data } 
    }
}

impl SellDesp for Vars
{
    fn res_info(&self) -> String
    {
        format!("vars")

    }
    fn res_name(&self) -> String
    {
        format!("vars")

    }
    fn res_allow(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
}



impl StartBehavior for Vars
{
    fn res_start(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
    fn res_conf(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
    fn res_check(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
}
impl StopBehavior for Vars 
{
    fn res_stop(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
    fn res_clean(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }

}
impl CallPlugin for Vars 
{
    fn res_before(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())
    }

    fn res_after(&self,_context : &mut Context) ->BoolR 
    {
        trace!("Vars::res_after") ;
        Ok(())

    }
}
