inner_use!() ;
#[derive(Debug)]
pub struct ResProxy
{
}

impl ResLoader<ResProxy> for   ResProxy
{
    fn load( _data : &StrMap) -> ResProxy
    {
        ResProxy{}
    }
    fn key() ->String { String::from("res")} 
}

impl ResDesp for ResProxy
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

impl InvokeStart for ResProxy
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
impl InvokeStop for ResProxy 
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
impl InvokeHook for ResProxy 
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
