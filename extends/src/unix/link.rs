use rg_lib::* ;
use rg_core::err::*   ;
use rg_core::def::*   ;
use rg_core::model::* ;
use rg_core::res::*   ;
use rg_core::creator::* ;
use std::path::Path ;

//
///!R.link
//      src : ""
//      dst : ""
//
//
#[derive(Debug)]
pub struct Link
{
    src: String,
    dst: String,

}
impl ResLoader<Link> for Link
{
    fn load( data : &StrMap) -> Link
    {
        let dst = data.must_get(&String::from("dst")).clone() ;
        let src = data.must_get(&String::from("src")).clone() ;
        Link{ src, dst} 
    }
    fn key() -> String { String::from("Link") }
}
impl ResDesp for Link
{
    fn res_info(&self) -> String {
        format!(
            r#"!R.link  
                    src: "{}"
                    dst: "{}"
                    "#,self.src,self.dst) 
    }
    fn res_name(&self) -> String {
        Link::key() 
    }
}

impl InvokeHook for Link{
    fn res_before(&self,context : &mut Context) ->BoolR 
    {
        let ee = EExpress::from_env_mix(context.to_map());
        context.set("src",ee.parse(&self.src));
        context.set("dst",ee.parse(&self.dst));
        Ok(())
    }

}
impl InvokeStop for Link{
    fn res_clean(&self,context : &mut Context) ->BoolR 
    {
        let dst : String = context.must_get("dst") ;
        let dst_path     = Path::new(dst.as_str()) ;
        if dst_path.exists() {
            self.clear_link(dst_path) ?
        }
        Ok(())
    }

}


impl Link
{
    fn clear_link(&self,dst_path: &Path) -> BoolR
    {
        if dst_path.read_link().is_ok()
        {
            let (code,stdout,stderr )= rg_sh!("unlink {}",dst_path.to_str().unwrap());
            if code != 0 {
                ERR!("{:?} {:?} ",stdout,stderr);
            } 
            Ok(())
        }
        else
        {
            ERR!("dst exists {:?}",dst_path);
        }
    }
}



impl InvokeStart for Link
{
    fn res_conf(&self,context : &mut Context) ->BoolR 
    {
        let dst : String = context.must_get("dst") ;
        let src : String = context.must_get("src") ;
        let dst_path = Path::new(dst.as_str()) ;
        let src_path = Path::new(src.as_str()) ;
        debug!("dst {:?}", dst_path);
        debug!("src {:?}", src_path);
        if dst_path.exists() {
            self.clear_link(dst_path) ?
        }
        if src_path.exists()
        {
            let parent = dst_path.parent();
            let f_name = dst_path.file_name() ;
            if let Some(p) = parent { 
                if let  Some(f) = f_name
                {
                    let (code,stdout,stderr )= rg_sh!("cd {:?} ; ln -s {} {}", 
                       p.to_str().unwrap(),src_path.to_str().unwrap(), f.to_str().unwrap()) ;
                    if code != 0 {
                        ERR!("{:?} {:?} ",stdout,stderr);
                    } 
                    return Ok(()) ;
                }
            }
        }
        ERR!("{} not exists",src_path.to_str().unwrap());
    }
}

pub fn  res_regist(f : &mut ResFatory)
{
    regist_res_creator::<Link>(f) ;
}

#[cfg(test)]
mod tests
{
    use super::* ;
    use std::env ;
    use pretty_env_logger ;
    #[test]
    fn creat_link()
    {
        pretty_env_logger::init();
        let mut god = ResFatory::new() ;
        res_regist(&mut god);
        let path = env::current_dir().unwrap();
        let mut c = Context::new() ;
        c.set("CUR_DIR",path.into_os_string().into_string().unwrap());
        let data  = map!(
            "dst" =>"${CUR_DIR}/meterial/run_ngx.yaml",
            "src" =>"${CUR_DIR}/meterial/run_tpl.yaml") ;
        let obj   = god.create(&Link::key(),&data ).unwrap();
        res_check(&obj,c);
    }
}
