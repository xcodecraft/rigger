inner_use!() ;
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

impl ResDesp for Vars
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



impl InvokeStart for Vars
{
    fn res_conf(&self,context : &mut Context) ->BoolR 
    {
        for (k,v) in &self.kvmap
        {
            context.sset(k.clone(),v.clone());
        }
        Ok(())

    }
}
impl InvokeStop for Vars { }
impl InvokeHook for Vars { }
