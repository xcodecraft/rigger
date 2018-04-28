use model::* ;
use def::{BoolR} ;

pub trait CmdDesp
{
    fn cmd_name(&self  ) ->String;
    fn cmd_info(&self  ) ->String;
}

#[macro_export]
macro_rules! cmd_info {
    { $cmd:expr  } => {
        print!("cmd {}: {:?}", $cmd.name(),$cmd.info()) ;
    };
}
#[macro_export]
macro_rules! cmd_assert {
    { $ret : expr  } => {
        assert!($ret.is_ok(),"err:{:?}",$ret);
    };
}

pub fn cmd_check<T>(res :&Box<Res>, cmd :&Box<T>) where T : Cmd + ?Sized
{
    let mut c = Context::new() ;
    let ret   = cmd.execute(res,&mut c) ;
    cmd_assert!(ret);
}



#[cfg(test)]
mod tests
{
    use super::* ;
    use res::* ;

    struct StubCmd {
    }
    impl StubCmd {
        fn new() -> StubCmd { StubCmd{} }
    }
    impl Cmd  for  StubCmd
    {
        fn execute(&self,res: &ResBox, context : &mut Context) -> BoolR
        {
            res.conf(context)? ;
            res.start(context)? ;
            Ok(())
        }
    }
    impl CmdDesp  for StubCmd
    {
        fn cmd_info(&self) -> String
        {
            format!("StubCmd: {}","")

        }
        fn cmd_name(&self) -> String
        {
            format!("StubCmd")

        }
    }

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
    fn usecmd_stub()
    {

        let cmd = Box::new(StubCmd::new());
        let res : ResBox = Box::new(StubRes::new());

        cmd_check(&res,&cmd) ;
    }
}
