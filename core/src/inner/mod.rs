
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
                RgvType::Env     => { Box::new(env::Env::load(data)  )    } ,
                RgvType::System  => { Box::new(sys::System::load(data) )  } ,
                RgvType::Project => { Box::new(prj::Project::load(data))  } ,
                RgvType::Res     => { Box::new(proxy::ResProxy::load(data)) } ,

            } ;
            //obj.build( parser);
            self.regist(obj);
        }
    }
    fn regist(&mut self, res : ResBox)
    {
        self.resvec_hold().push(res);
    }
}

