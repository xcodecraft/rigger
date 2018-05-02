use rg_lib::* ;
use rg_core::err::*   ;
use rg_core::def::*   ;
use rg_core::model::* ;
use rg_core::res::*   ;
use rg_core::creator::* ;
//use std::path::Path ;

//
///!R.path
//      dst: "/a,/b,/c"
#[derive(Debug)]
pub struct Path
{
    dst: String,

}
impl ResLoader<Path> for Path
{
    fn load( data : &StrMap) -> Path
    {
        let dst = data.must_get(&String::from("dst")).clone() ;
        Path{ dst} 
    }
    fn key() -> String { String::from("Path") }
}
impl ResDesp for Path
{
    fn res_info(&self) -> String {
        format!(
            r#"!R.path
                    dst: "{}"
                    "#,self.dst) 
    }
    fn res_name(&self) -> String {
        Path::key() 
    }
}

impl InvokeHook for Path{
    fn res_before(&self,context : &mut Context) ->BoolR 
    {
        let ee = EExpress::from_env_mix(context.to_map());
        context.set("dst",ee.parse(&self.dst));
        Ok(())
    }

}
impl InvokeStop for Path{
    fn res_clean(&self,context : &mut Context) ->BoolR 
    {

        let line: String = context.must_get("dst") ;
        let dst_v: Vec<&str> = line.split(',').collect();
        for dst in dst_v
        {
            let (code,stdout,stderr )= rg_sh!("rm -rf {}", dst ) ;
            if code != 0 {
                ERR!("{:?} {:?} ",stdout,stderr);
            } 
        }
        return Ok(()) ;
    }

}


impl Path
{
}



impl InvokeStart for Path
{
    fn res_conf(&self,context : &mut Context) ->BoolR 
    {
        let line: String = context.must_get("dst") ;
        let dst_v: Vec<&str> = line.split(',').collect();
        for dst in dst_v
        {
            let (code,stdout,stderr )= rg_sh!("mkdir -p {}", dst ) ;
            if code != 0 {
                ERR!("{:?} {:?} ",stdout,stderr);
            } 
        }
        return Ok(()) ;
    }
}

pub fn  res_regist(f : &mut ResFatory)
{
    regist_res_creator::<Path>(f) ;
}

#[cfg(test)]
mod tests
{
    use super::* ;
    use std::env ;
    #[test]
    fn creat_path()
    {
        let mut god = ResFatory::new() ;
        res_regist(&mut god);
        let path  = env::current_dir().unwrap();
        let mut c = Context::new() ;
        c.set("CUR_DIR",path.into_os_string().into_string().unwrap());
        let data  = map!(
            "dist" =>"${CUR_DIR}/meterial/a,${CUR_DIR}/meterial/b",
            );
        let obj   = god.create(&Path::key(),&data ).unwrap();
        res_check(&obj,c);
    }
}
