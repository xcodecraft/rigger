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


#[macro_use]
pub mod err ;
#[macro_use]
pub mod def ;
pub mod model ;
pub mod creator ;
pub mod parser ;
pub mod res ;
pub mod inner ;


use err::* ;
use def::* ;
use model::* ;
use parser::* ;
use creator::* ;
use inner::rgm::* ;
use inner::*;

pub fn rg_main()
{
    let mut data = vec![
        ParseResult::inn( RgvType::Env    , map!( "_name" => "dev" ))  ,
        ParseResult::inn( RgvType::Vars   , map!( "x"     => "256"     , "y" => "24")) ,
        ParseResult::inn( res_of("Echo")  , map!( "value" => "china")) ,
        ParseResult::end()                ,
        ParseResult::inn( RgvType::System , map!( "_name" => "api" ) ) ,
        ParseResult::inn( RgvType::Vars   , map!( "x"     => "256"     , "y" => "24")) ,
        ParseResult::end()                ,
    ];

    let parser : ParserBox = Box::new(StubParser::new(data)) ;
    let mut god = ResFatory::new() ;
    mod_regist(&mut god);
    let mut context        = Context::new();
    let mut main           = RGMain::new() ;
    main.build(&parser,&god) ;
    debug!("main: {:?}", main) ;
    main.conf(&mut context) ;

}
