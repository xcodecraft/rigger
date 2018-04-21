inner_use!();
#[derive(Debug)]
pub struct System
{
    name   : String,
    limit  : String,
    resvec : RefCell<ResVec>,

}
impl System 
{
    pub fn new(name : String) -> System
    {
        let resvec = RefCell::new(ResVec::new()) ;
        System{name,  limit: String::new(),resvec  }
    }
    pub fn load(data: StrMap ) ->System
    {
        let name = data.must_get(&String::from("_name")) ;
        let sys  = System::new(name.clone()) ;
        return sys ;

    }

}
impl SellDesp for System
{
    fn res_info(&self) -> String
    {
        format!("System: {}",self.name)

    }
    fn res_name(&self) -> String
    {
        format!("System: {}",self.name)

    }
    fn res_allow(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
}


impl CallPlugin for System 
{
    fn res_before(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())
    }

    fn res_after(&self,_context : &mut Context) ->BoolR 
    {
        trace!("System::res_after") ;
        Ok(())

    }
}
impl InnerContainer for System {

    fn resvec_hold(&self) ->RefMut<ResVec>
    {
        self.resvec.borrow_mut()
    }
}
