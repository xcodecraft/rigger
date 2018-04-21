
use model::* ;
use def::* ;
use res::* ;
use super::* ;

impl Env
{
    pub fn new(name : String) -> Env
    {
        Env{name, resvec : ResVec::new(), mix : Vec::new() }
    }
    pub fn load(data: StrMap ) ->Env
    {
        let name = data.must_get(&String::from("_name")) ;
        let mut env  = Env::new(name.clone()) ;
        data.get(&String::from("_mix")).ok_or(0)
            .and_then(|mix | Ok(env.mix.push(mix.clone()))) ; 
        return env ;

    }

}

impl SellDesp for Env
{
    fn res_info(&self) -> String
    {
        format!("env: {}",self.name)

    }
    fn res_name(&self) -> String
    {
        format!("env: {}",self.name)

    }
    fn res_allow(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
}
impl CallPlugin for Env 
{
    fn res_before(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())
    }

    fn res_after(&self,_context : &mut Context) ->BoolR 
    {
        trace!("Env::res_after") ;
        Ok(())

    }
}
