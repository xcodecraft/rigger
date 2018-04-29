use def::* ;
use model::* ;

pub trait ResDesp
{
    fn res_name(&self  ) ->String;
    fn res_info(&self  ) ->String;
    fn res_allow(&self,_context : &mut Context) ->BoolR { Ok(()) }
}
pub trait InvokeHook 
{
    fn res_before(&self,_context : &mut Context) ->BoolR { Ok(()) }
    fn res_after(&self,_context  : &mut Context) ->BoolR { Ok(()) }
}
pub trait InvokeStart
{
    fn res_start(&self,_context : &mut Context) ->BoolR { Ok(()) }
    fn res_conf(&self,_context  : &mut Context) ->BoolR { Ok(()) }
    fn res_check(&self,_context : &mut Context) ->BoolR { Ok(()) }
}

pub trait InvokeStop
{
    fn res_stop(&self,_context  : &mut Context) ->BoolR { Ok(()) }
    fn res_clean(&self,_context : &mut Context) ->BoolR { Ok(()) }
}
pub trait  Interceptor
{
    fn do_before(&self , context : &mut Context ) ->BoolR ;
    fn do_after(&self  , context : &mut Context ) ->BoolR ;
}


impl <T> Interceptor for T  where T : InvokeHook +  ResDesp
{
    fn do_before(&self , context : &mut Context ) ->BoolR
    {
        trace!("[{}]::res_before", self.res_name()) ;
        self.res_before(context)
    }
    fn do_after(&self  , context : &mut Context ) ->BoolR
    {
        trace!("[{}]::res_after", self.res_name()) ;
        self.res_after(context)
    }
}


impl<T> Res for T  
where T: ResDesp +  InvokeStart + InvokeStop + InvokeHook + Interceptor
{

    fn allow(&self,context : &mut Context) ->BoolR 
    {
        self.res_allow(context) 
    }
    fn conf(&self,context : &mut Context) ->BoolR 
    {
        self.do_before(context)? ;
        trace!("[{}]::conf", self.res_name()) ;
        self.res_conf(context)? ;
        self.do_after(context)?;
        Ok(())
    }

    fn start(&self,context : &mut Context) ->BoolR 
    {
        self.do_before(context)? ;
        trace!("[{}]::start", self.res_name()) ;
        self.res_start(context)? ;
        self.do_after(context)?;
        Ok(())
    }

    fn stop(&self,context : &mut Context) ->BoolR 
    {
        self.do_before(context)? ;
        trace!("[{}]::stop", self.res_name()) ;
        self.res_stop(context)? ;
        self.do_after(context)? ;
        Ok(())
    }
    fn check(&self,context : &mut Context) ->BoolR 
    {
        self.do_before(context)? ;
        trace!("[{}]::check", self.res_name()) ;
        self.res_check(context)? ;
        self.do_after(context)? ;
        Ok(())

    }
    fn clean(&self,context : &mut Context) ->BoolR 
    {
        self.do_before(context)? ;
        trace!("[{}]::clean", self.res_name()) ;
        self.res_clean(context)? ;
        self.do_after(context)? ;
        Ok(())

    }
    fn info(&self) ->String
    {
        self.res_info()
    }
    fn name(&self) ->String
    {
        self.res_name()
    }
}
#[macro_export]
macro_rules! res_info {
    { $res:expr  } => {
        print!("res {}: {:?}", $res.name(),$res.info()) ;
    };
}
#[macro_export]
macro_rules! res_assert {
    { $ret : expr  } => {
        assert!($ret.is_ok(),"err:{:?}",$ret);
    };
}

pub fn res_check<T>( res :&Box<T>, mut c : Context) where T : Res + ?Sized 
{
    res_info!(res);
    res_assert!(res.conf(  &mut c)) ;
    res_assert!(res.start( &mut c));
    res_assert!(res.stop(  &mut c)) ;
    res_assert!(res.clean( &mut c)) ;
}





#[cfg(test)]
mod tests
{
    use super::* ;

    struct StubRes {
    }
    impl StubRes {
        fn new() -> StubRes { StubRes{} }
    }
    impl ResDesp  for StubRes
    {
        fn res_info(&self) -> String
        {
            format!("StubRes: {}","")

        }
        fn res_name(&self) -> String
        {
            format!("StubRes: {}","")

        }
        fn res_allow(&self,_context : &mut Context) ->BoolR 
        {
            Ok(())

        }

    }
    impl InvokeHook for StubRes {}
    impl InvokeStart for StubRes {}
    impl InvokeStop for StubRes {}
    #[test]
    fn useres_stub()
    {

        let res = Box::new(StubRes::new());
        let c =  Context::new();
        res_check(&res,c) ;
    }
}
