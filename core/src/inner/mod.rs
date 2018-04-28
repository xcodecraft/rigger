
use model::* ;
use res::* ;
use def::* ;
use std::cell::RefMut;
use creator::* ;
use parser::* ;


#[macro_export]
macro_rules! inner_use{
  ()  =>   {
      #[allow(unused_imports)]
      use rg_lib::* ;
      #[allow(unused_imports)]
      use model::* ;
      #[allow(unused_imports)]
      use def::* ;
      #[allow(unused_imports)]
      use res::* ;
      #[allow(unused_imports)]
      use creator::* ;
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

mod env ;
mod prj ;
pub mod rgm ;
mod sys ;
mod var ;
mod proxy ;
mod utls ;


impl <T> InvokeStart for T where T: InnerContainer
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
impl <T> InvokeStop for T where T: InnerContainer
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


pub trait Compose
{
    fn build(&mut self,parser : &ParserBox,god :&ResFatory) ;
    fn regist(&mut self, res : ResBox);
}
impl <T> Compose for T  where T: InnerContainer 
{
    fn build(&mut self,parser : &ParserBox,god :&ResFatory)
    {
        loop {
            if let Some(x) = parser.next() 
            {
                match  x.state
                {
                    ParseState::In  => {

                        if let Some(v) = x.node 
                        {

                            let obj: ResBox = match v.rkey
                            {
                                RgvType::Vars    => { Box::new(var::Vars::load(v.data) )    } ,
                                RgvType::Env     => { 
                                    let mut obj = Box::new(env::Env::load(&v.data)); 
                                    obj.build(parser,god); 
                                    obj   
                                } ,
                                RgvType::System  => { 
                                    let mut obj = Box::new(sys::System::load(v.data) ) ;
                                    obj.build(parser,god) ;
                                    obj 
                                } ,
                                RgvType::Project => { 
                                    let mut obj =  Box::new(prj::Project::load(v.data));  
                                    obj.build(parser,god) ;
                                    obj

                                } ,
                                RgvType::Res(key) => { god.create(&key,&v.data).unwrap() } ,

                            } ;
                            trace!("regist {}", obj.info() );
                            self.regist(obj);
                        }
                    },
                    ParseState::End =>  { break; }
                }
            }
            else { break; }
        }
    }
    fn regist(&mut self, res : ResBox)
    {
        self.resvec_hold().push(res);
    }
}


pub fn  mod_res_regist(f : &mut ResFatory)
{
    utls::res_regist(f) ;
}


