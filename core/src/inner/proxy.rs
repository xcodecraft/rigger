
use model::* ;
use def::* ;
use res::* ;
#[derive(Debug)]
pub struct ResProxy
{
}

impl  ResProxy
{
    pub fn load( data : StrMap) -> ResProxy
    {
        ResProxy{}
    }
}

impl SellDesp for ResProxy
{
    fn res_info(&self) -> String
    {
        format!("ResProxy : {}","")

    }
    fn res_name(&self) -> String
    {
        format!("ResProxy: {}","")

    }
    fn res_allow(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
}

impl StartBehavior for ResProxy
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
impl StopBehavior for ResProxy 
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
impl CallPlugin for ResProxy 
{
    fn res_before(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())
    }

    fn res_after(&self,_context : &mut Context) ->BoolR 
    {
        trace!("ResProxy::res_after") ;
        Ok(())

    }
}
