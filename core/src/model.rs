use std::collections::HashMap;
use err ;
use def::* ;
pub struct Context
{

}
impl Context 
{
    pub fn keep(&self) 
    {

    }
    pub fn rollback(&self)
    {
    }


}

pub trait CliCmd
{

}
pub trait CallPlugin 
{
    fn res_before(&self ) ->BoolR;
    fn res_after(&self  ) ->BoolR;
}
pub trait StopBehavior
{
    fn res_stop(&self   ) ->BoolR;
    fn res_check(&self  ) ->BoolR;
    fn res_clean(&self  ) ->BoolR;
}
pub trait StartBehavior
{
    fn res_allow(&self  ) ->BoolR;
    fn res_conf(&self   ) ->BoolR;
    fn res_start(&self  ) ->BoolR;
    fn res_info(&self   ) ->BoolR;
}

pub trait Res
{
    fn allow(&self  ) ->BoolR;
    fn conf(&self) ->BoolR ;
    fn start(&self  ) ->BoolR;
    fn stop(&self   ) ->BoolR;
    fn check(&self  ) ->BoolR;
    fn clean(&self  ) ->BoolR;
    fn info(&self   ) ->BoolR;
}
pub type ResBox = Box<Res> ;

type ResMap = HashMap<String,Box<Res>> ;
pub struct ResCollection
{
    resmap : ResMap ,
}

impl<T> Res for T  
where T: StartBehavior + StopBehavior + CallPlugin
{
    fn allow(&self) ->BoolR 
    {
        self.res_allow() 
    }
    fn conf(&self) ->BoolR 
    {
        self.res_before()? ;
        self.res_conf()? ;
        self.res_after()?;
        Ok(())
    }

    fn start(&self) ->BoolR 
    {
        self.res_before()? ;
        self.res_start()? ;
        self.res_after()?;
        Ok(())
    }

    fn stop(&self) ->BoolR 
    {
        self.res_before()? ;
        self.res_stop()? ;
        self.res_after()? ;
        Ok(())
    }
    fn check(&self) ->BoolR 
    {
        self.res_before()? ;
        self.res_check()? ;
        self.res_after()? ;
        Ok(())

    }
    fn clean(&self) ->BoolR 
    {
        self.res_before()? ;
        self.res_clean()? ;
        self.res_after()? ;
        Ok(())

    }
    fn info(&self) ->BoolR 
    {
        self.res_before()? ;
        self.res_info()? ;
        self.res_after()? ;
        Ok(())
    }
}


#[cfg(test)]
mod tests
{
    use super::* ;
    use pretty_env_logger ;
    struct StubRes {
    }
    impl StubRes {
        fn new() -> StubRes { StubRes{} }
    }
    //impl DefaultCtrl for StubRes {} 
    impl StartBehavior for StubRes
    {
        fn res_allow(&self) ->BoolR 
        {
            Ok(())

        }
        fn res_start(&self) ->BoolR 
        {
            Ok(())

        }
        fn res_conf(&self) ->BoolR 
        {
            Ok(())

        }
        fn res_info(&self) ->BoolR 
        {
            Ok(())

        }
    }
    impl StopBehavior for StubRes 
    {
        fn res_stop(&self) ->BoolR 
        {
            Ok(())

        }
        fn res_clean(&self) ->BoolR 
        {
            Ok(())

        }
        fn res_check(&self) ->BoolR 
        {
            Ok(())

        }

    }
    impl CallPlugin for StubRes 
    {
        fn res_before(&self) ->BoolR 
        {
            Ok(())
        }

        fn res_after(&self) ->BoolR 
        {
            trace!("StubRes::res_after") ;
            Ok(())

        }
    }
    fn resres_check<T>( res :&T) where T : Res
    {
        assert!(res.info().is_ok()) ;
        assert!(res.conf().is_ok()) ;
        assert!(res.start().is_ok());
        assert!(res.stop().is_ok()) ;
        assert!(res.clean().is_ok()) ;
    }
    #[test]
    fn useres_stub()
    {

        pretty_env_logger::init();
        let res = StubRes::new();
        resres_check(&res) ;
    }
}
