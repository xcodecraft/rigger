#[macro_use] extern crate log;
#[allow(unused_imports)]
extern crate pretty_env_logger;
extern crate regex;
#[allow(unused_imports)]
extern crate shells ;

mod def ;
#[macro_use] pub mod macros ;
mod yaml;
mod eexp ;

pub use def::StrMap ;
pub use eexp::EExpress ;


