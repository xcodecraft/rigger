inner_use!() ;
#[derive(Debug)]
pub struct Env
{
    name   : String,
    resvec : RefCell<ResVec>,
    mix    : Vec<String>,
}


impl Env
{
    pub fn new(name : String) -> Env
    {
        Env{name, resvec : RefCell::new(ResVec::new()), mix : Vec::new() }
    }

}
impl Loader<Env> for Env
{

    fn load(data: &StrMap ) ->Env
    {
        let name    = data.must_get(&String::from("_name")) ;
        let mut env = Env::new(name.clone()) ;
        data.get(&String::from("_mix")).ok_or(0)
            .and_then(|mix | Ok(env.mix.push(mix.clone()))) ; 
        return env ;

    }
    fn key() -> String { String::from("Env") }
}

impl SellDesp for Env
{
    fn res_info(&self) -> String
    {
        format!("env: {}",self.name)

    }
    fn res_name(&self) -> String
    {
        format!("Env::{}",self.name)

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
        Ok(())

    }
}
impl InnerContainer for Env {

    fn resvec_hold(&self) ->RefMut<ResVec>
    {
        self.resvec.borrow_mut()
    }
}
