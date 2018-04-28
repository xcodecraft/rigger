use model::* ;
use res::* ;
use cmd::* ;
use def::* ;
use std::cell::RefMut;
use creator::* ;
use parser::* ;


#[macro_export]
macro_rules! cmd_use{
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

pub fn  mod_cmd_regist(f : &mut CmdFatory)
{
    run::cmd_regist(f) ;
}

pub mod run ;
