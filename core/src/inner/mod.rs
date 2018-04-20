
use model::* ;

pub trait InnerContainer {
    fn resvec_hold<'a>(&'a mut self) ->&'a mut  ResVec ;
}

// pub trait ResCollection
// {
    // fn regist(&mut self,res ResBox) ;
// }
// impl <T>  ResCollection  for T  where T : InnerContainer
// {
    // fn regist(&mut self,res ResBox)
    // {
//
//
    // }
// }

pub  struct  Keyword
{
    //const  name : String =  String::from("_name") ;
}

pub struct Modul
{

}

mod env ;
mod prj ;
mod rgm ;
mod sys ;
mod var ;
mod proxy ;

// impl InnerContainer for env::Env {}
// impl InnerContainer for sys::System{}
// impl InnerContainer for prj::Project{}
// impl InnerContainer for rgm::RGMain{}


/*
impl <T> StartBehavior for T where T: InnerContainer
{
    fn res_allow(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
    fn res_start(&self,_context : &mut Context) ->BoolR 
    {

        let resvec = self.resvec_hold() ;
        for res  in resvec 
        {
            res.start()? ;

        }
        Ok(())

    }
    fn res_conf(&self,_context : &mut Context) ->BoolR 
    {
        let resvec = self.resvec_hold() ;
        for res  in resvec 
        {
            res.conf()? ;

        }
        Ok(())

    }
    fn res_info(&self) -> String
    {
        format!("env: {}",self.name)

    }
}
impl <T> StopBehavior for T where T: InnerContainer
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
*/


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

