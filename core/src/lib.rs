#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rg_lib ;

extern crate toml;


#[macro_use]
pub mod err ;
#[macro_use]
pub mod def ;
pub mod model ;
pub mod creator ;
pub mod parser ;
pub mod res ;
pub mod cmd ;
pub mod cmds ;
pub mod inner ;


use err::* ;
use def::* ;
use model::* ;
use parser::* ;
use creator::* ;
use inner::rgm::* ;
use inner::*;
use cmd:: * ;
use cmds::* ;

#[derive(Debug,Clone)]
pub struct RGArgs
{
    pub  env : String,
    pub  sys : String,
    pub  act : String,
}
impl RGArgs
{
    pub fn new() -> RGArgs
    {
        RGArgs{ env : String::new(), sys : String::new() , act: String::new() }
    }

}
pub struct Rigger
{
    parser     : ParserBox,
    res_factor : ResFatory,
    cmd_factor : CmdFatory,

}
impl Rigger{

    pub fn new() -> Rigger
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
        let mut res_factor     = ResFatory::new() ;
        let mut cmd_factor     = CmdFatory::new() ;
        mod_cmd_regist(&mut cmd_factor);
        mod_res_regist(&mut res_factor);
        Rigger{ parser, res_factor, cmd_factor}
    }

    pub fn load(&self, main : &mut RGMain ,_conf_file : &String)
    {
        main.build(&self.parser,&self.res_factor) ;
    }
    pub fn run(&mut self, main :&ResBox,line: String) ->BoolR
    {
        let mut context     = Context::new();
        let cmds: Vec<&str> = line.split(',').collect();
        for c in cmds 
        {
             if let Some(cmd) =  self.cmd_factor.create(&String::from(c))
             {
                 cmd.execute(main,&mut context) ? ;
             }
        }
        Ok(())

    }
}
pub fn rg_main( args : RGArgs )
{
    let mut main = Box::new(RGMain::new()) ;
    let mut rg   = Rigger::new() ;
    rg.load(&mut main,&format!("")) ;
    let res :ResBox = main ;
    rg.run(&res,args.act) ;
}
