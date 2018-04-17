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
pub trait Icontrol {
    fn _allow(&self  ) ->BoolR;
    fn _before(&self ) ->BoolR;
    fn _after(&self  ) ->BoolR;
    fn _start(&self  ) ->BoolR;
    fn _stop(&self   ) ->BoolR;
    fn _check(&self  ) ->BoolR;
    fn _clean(&self  ) ->BoolR;
    fn _info(&self   ) ->BoolR;
}
pub trait DefaultCtrl
{

}

pub trait Res
{
    fn allow(&self  ) ->BoolR;
    fn start(&self  ) ->BoolR;
    fn stop(&self   ) ->BoolR;
    fn check(&self  ) ->BoolR;
    fn clean(&self  ) ->BoolR;
    fn info(&self   ) ->BoolR;
}

type ResMap = HashMap<String,Box<Res>> ;
pub struct ResCollection
{
    resmap : ResMap ,
}

impl<T> Res for T  
    where T: Icontrol
{
    fn allow(&self) ->BoolR 
    {
        self._allow() 
    }
    fn start(&self) ->BoolR 
    {
        self._before()? ;
        self._start()? ;
        self._after()?;
        Ok(())
    }

    fn stop(&self) ->BoolR 
    {
        self._before()? ;
        self._stop()? ;
        self._after()? ;
        Ok(())
    }
    fn check(&self) ->BoolR 
    {
        self._before()? ;
        self._check()? ;
        self._after()? ;
        Ok(())

    }
    fn clean(&self) ->BoolR 
    {
        self._before()? ;
        self._clean()? ;
        self._after()? ;
        Ok(())

    }
    fn info(&self) ->BoolR 
    {
        self._before()? ;
        self._info()? ;
        self._after()? ;
        Ok(())
    }

}
impl <T> Icontrol for  T 
 where T : DefaultCtrl
{
    fn _allow(&self)  -> BoolR
    {
        trace!("DefaultCtrl::_allow") ;
        Ok(())
    }
    fn _before(&self) -> BoolR
    {
        trace!("DefaultCtrl::_before") ;
        Ok(())
    }
    fn _after(&self) -> BoolR
    {
        trace!("DefaultCtrl::_after") ;
        Ok(())

    }
    fn _start(&self) -> BoolR
    {
        trace!("DefaultCtrl::_start") ;
        Ok(())

    }
    fn _stop(&self) -> BoolR
    {
        trace!("DefaultCtrl::_stop") ;
        Ok(())

    }
    fn _check(&self) -> BoolR
    {
        trace!("DefaultCtrl::_check") ;
        Ok(())

    }
    fn _clean(&self) -> BoolR
    {
        trace!("DefaultCtrl::_clean") ;
        Ok(())

    }
    fn _info(&self) -> BoolR
    {
        trace!("DefaultCtrl::_info") ;
        Ok(())

    }
}

#[cfg(test)]
mod tests
{
    use super::* ;
    struct StubRes {

    }
    //impl DefaultCtrl for StubRes {} 
    impl Icontrol for StubRes
    {

        fn _allow(&self) ->BoolR 
        {

         Ok(())

        }
        fn _before(&self) ->BoolR 
        {

         Ok(())

        }
        fn _after(&self) ->BoolR 
        {
            trace!("StubRes::_after") ;
            Ok(())

        }
        fn _start(&self) ->BoolR 
        {
         Ok(())

        }
        fn _stop(&self) ->BoolR 
        {
         Ok(())

        }
        fn _check(&self) ->BoolR 
        {
         Ok(())

        }
        fn _clean(&self) ->BoolR 
        {
         Ok(())

        }
        fn _info(&self) ->BoolR 
        {
         Ok(())

        }
    }

    use pretty_env_logger ;
    #[test]
    fn use_stub()
    {

        pretty_env_logger::init();
        let res = StubRes{} ;
        res.start() ;
    }
}
