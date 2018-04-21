
use model::* ;
use res::* ;
use def::* ;
use std::cell::RefMut;


#[macro_export]
macro_rules! inner_use{
  ()  =>   {
      #[allow(unused_imports)]
      use model::* ;
      #[allow(unused_imports)]
      use def::* ;
      #[allow(unused_imports)]
      use res::* ;
      #[allow(unused_imports)]
      use super::* ;
      #[allow(unused_imports)]
      use std::cell::{ RefCell,RefMut} ;
    }
}


pub  struct  Keyword
{
    //const  name : String =  String::from("_name") ;
}

pub trait InnerContainer {
    fn resvec_hold(& self) ->RefMut<ResVec> ;
}

pub mod env ;
pub mod prj ;
pub mod rgm ;
pub mod sys ;
pub mod var ;
pub mod proxy ;
pub mod bind ;


impl <T> StartBehavior for T where T: InnerContainer
{
    fn res_start(&self,context : &mut Context) ->BoolR 
    {

        let resvec = &*self.resvec_hold() ;
        for res  in resvec { res.start(context)? ; }
        Ok(())

    }
    fn res_conf(&self,context : &mut Context) ->BoolR 
    {
        let resvec = &*self.resvec_hold() ;
        for res  in resvec { res.conf(context)? ; }
        Ok(())

    }
    fn res_check(&self,context : &mut Context) ->BoolR 
    {
        let resvec = &*self.resvec_hold() ;
        for res in resvec { res.check(context) ; }
        Ok(())

    }
}
impl <T> StopBehavior for T where T: InnerContainer
{
    fn res_stop(&self,context : &mut Context) ->BoolR 
    {
        let resvec = &*self.resvec_hold() ;
        //resvec.reverse();
        for res  in resvec { res.stop(context)? ; }
        Ok(())

    }
    fn res_clean(&self,context : &mut Context) ->BoolR 
    {
        let resvec = &*self.resvec_hold();
        //resvec.reverse();
        for res in resvec { res.clean(context) ; }
        Ok(())

    }

}


trait Compose
{
    fn build(&mut self,parser : &ParserBox) ;
    fn regist(&mut self, res : ResBox);
}
impl <T> Compose for T  where T: InnerContainer 
{
    fn build(&mut self,parser : &ParserBox)
    {
        if let Some((v,data)) = parser.next() 
        {

            let obj: ResBox = match v
            {
                RgvType::Vars    => { Box::new(var::Vars::load(data) )    } ,
                RgvType::Env     => { 
                    let mut obj = Box::new(env::Env::load(data)); 
                                       obj.build(parser); 
                                       obj   
                } ,
                RgvType::System  => { 
                    let mut obj = Box::new(sys::System::load(data) ) ;
                    obj.build(parser) ;
                    obj 
                } ,
                RgvType::Project => { 
                    let mut obj =  Box::new(prj::Project::load(data));  
                    obj.build(parser) ;
                    obj

                } ,
                RgvType::Res     => { Box::new(proxy::ResProxy::load(data)) } ,

            } ;
            trace!("regist {}", obj.info() );
            //obj.build( parser);
            self.regist(obj);
        }
    }
    fn regist(&mut self, res : ResBox)
    {
        self.resvec_hold().push(res);
    }
}




