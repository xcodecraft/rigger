
use model::* ;
use def::* ;
use res::* ;
use super::* ;
#[derive(Debug)]
pub struct Env
{
    name   : String,
    resvec : ResVec,
    mix    : Vec<String>,

}

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
impl InnerContainer for Env {

    fn resvec_hold<'a>(&'a mut self) ->&'a mut  ResVec 
    {
        &mut self.resvec
    }
}

impl StartBehavior for Env
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
    fn res_info(&self) -> String
    {
        format!("env: {}",self.name)

    }
}
impl StopBehavior for Env 
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
