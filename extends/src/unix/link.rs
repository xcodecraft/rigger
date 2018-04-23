use rg_core::err::*   ;
use rg_core::def::*   ;
use rg_core::model::* ;
use rg_core::res::*   ;
use rg_core::creator::* ;
use std::path::Path ;

#[derive(Debug)]
pub struct Link
{
    src: String,
    dst: String,

}
impl Loader<Link> for Link
{
    fn load( data : &StrMap) -> Link
    {
        let dst = data.must_get(&String::from("dst")).clone() ;
        let src = data.must_get(&String::from("src")).clone() ;
        Link{ src, dst} 
    }
    fn key() -> String { String::from("Link") }
}



impl InvokeHook for Link{}
impl InvokeStop for Link{

}


impl ResDesp for Link
{
    fn res_info(&self) -> String {
        format!("{:?}",self)
    }
    fn res_name(&self) -> String {
        Link::key() 
    }
}


#[macro_export]
macro_rules! ERR{
    ($args:expr) => ({ return Error::Runtime(String::from($args)) });
    ($fmt:expr, $($args:tt)*) => ({  
        return Err(Error::Runtime(format!($fmt,$($args)*)))
    });
}


impl InvokeStart for Link
{
    fn res_conf(&self,_context : &mut Context) ->BoolR 
    {
        let dst_path = Path::new(self.dst.as_str()) ;
        let src_path = Path::new(self.src.as_str()) ;
        if dst_path.exists() 
        {
            if dst_path.read_link().is_ok()
            {
                let (code,stdout,stderr )= sh!("unlink {}",self.dst);
                if code != 0 {
                    ERR!("{:?} {:?} ",stdout,stderr);
                } 
            }
            else
            {
                ERR!("dst exists {:?}",self.dst);
            }
        }
        //TODO: 
        /*
        if src_path.exists()
        {
            let parent = dst_path.parent();
            let f_name = dst_path.file_name() ;
            if let Some(p) = parent { 
                if let  Some(f) = f_name
                {
                    let (code,stdout,stderr )= sh!("cd {:?} ; ln -s {} {:?}", p.as_os_str(),self.src, f) ;
                    if code != 0 {
                        ERR!("{:?} {:?} ",stdout,stderr);
                    } 
                    return Ok(()) ;
                }
            }
        }
        ERR!("{} not exists",self.src);
        */
        Ok(())

    }
}



pub fn  res_regist(f : &mut ResFatory)
{
    regist_creator::<Link>(f) ;
}

#[cfg(test)]
mod tests
{
    use super::* ;
    //use rg_core::res::* ;
    use rg_core::model::* ;
    use rg_core::parser::* ;
    use rg_core::creator::* ;
    use pretty_env_logger ;
    #[test]
    fn creat_link()
    {
       pretty_env_logger::init();
       debug!("hello") ;
       let mut god = ResFatory::new() ;
       res_regist(&mut god);
       let data = map!("dst"  =>"${HOME}/devspace/rigger-nx/extends/meterial/run_ngx.yaml","src" => "${HOME}/devspace/rigger-nx/extends/meterial/run_tpl.yaml") ;
       let obj  = god.create(&Link::key(),&data ).unwrap();
       res_check(&obj);
    }
}
