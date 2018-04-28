inner_use!() ;


// !R.echo 
//      value : "${PRJ}"
//
//
pub struct Echo
{
    value : String

}
impl ResLoader<Echo> for Echo
{
    fn load( data : &StrMap) -> Echo
    {
        let value = data.must_get(&String::from("value")).clone() ;
        Echo{ value  } 
    }
    fn key() -> String { Keyword::Echo.name() }
}



impl InvokeHook for Echo{}
impl InvokeStop for Echo{}


impl ResDesp for Echo
{
    fn res_info(&self) -> String {
        format!("Echo")
    }
    fn res_name(&self) -> String {
        format!("Echo")
    }
}



impl InvokeStart for Echo
{
    fn res_conf(&self,_context : &mut Context) ->BoolR 
    {
        println!("{}",self.value) ;
        Ok(())
    }
}


#[derive(Debug)]
enum Keyword
{
    Echo,
}

impl Keyword
{
    pub fn name(&self) -> String {
        format!("{:?}",self)
    }
}


pub fn  res_regist(f : &mut ResFatory)
{
    regist_res_creator::<Echo>(f) ;
}

#[cfg(test)]
mod tests
{
    use super::* ;
    use model::* ;
    use parser::* ;
    #[test]
    fn creat_echo()
    {
       let mut god = ResFatory::new() ;
       mod_res_regist(&mut god);
       let data = map!("value" =>"china") ;
       let obj   = god.create(&String::from("Echo"),&data ).unwrap();
       let mut c = Context::new();
       obj.conf(&mut c);
    }
}
