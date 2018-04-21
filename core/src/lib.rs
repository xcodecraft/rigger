#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

extern crate toml;

#[macro_export]
macro_rules! use_min_lib {
  ()  =>   {

      #[allow(unused_imports)]
      use std ;
      #[allow(unused_imports)]
      use err ;
      #[allow(unused_imports)]
      use def::* ;
  }
}

#[macro_export]
macro_rules! use_lib {
  ()  =>   {

      #[allow(unused_imports)]
      use std ;
      #[allow(unused_imports)]
      use err ;
      #[allow(unused_imports)]
      use err::ResultFlow ;
      #[allow(unused_imports)]
      use def::* ;
      #[allow(unused_imports)]
      use model::* ;
  }
}

#[macro_export]
macro_rules! use_test {
  ()  =>   {

      #[allow(unused_imports)]
      use std ;
      #[allow(unused_imports)]
      use file;
      #[allow(unused_imports)]
      use err ;
      #[allow(unused_imports)]
      use def::* ;
      #[allow(unused_imports)]
      use model::* ;
    }
}

#[macro_use]
mod err ;
#[macro_use]
mod def ;
pub mod model ;
pub mod creator ;
mod parser ;
mod res ;
mod inner ;


pub fn __main() {
    assert!(false) ;
    debug!("hello") ;
}

