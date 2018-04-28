inner_use!();
#[derive(Debug)]
pub struct RGMain
{
    resvec : RefCell<ResVec>,
}

impl RGMain 
{
    pub fn new( ) ->RGMain
    {
        RGMain{ resvec : RefCell::new(ResVec::new()) }
    }
}
impl InnerContainer for RGMain {

    fn resvec_hold(&self) ->RefMut<ResVec>
    {
        self.resvec.borrow_mut()
    }
}

impl ResDesp for RGMain
{
    fn res_info(&self) -> String
    {
        format!("RG Main: {}","")

    }
    fn res_name(&self) -> String
    {
        format!("RG Main") 

    }
    fn res_allow(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
}


impl InvokeHook for RGMain
{
    fn res_before(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())
    }

    fn res_after(&self,_context : &mut Context) ->BoolR 
    {
        Ok(())

    }
}



#[cfg(test)]
mod tests
{
    use super::* ;
    use model::* ;
    use parser::* ;
    use std::cell::RefCell;
    use pretty_env_logger ;
    #[test]
    fn use_it()
    {
        pretty_env_logger::init();
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
        mod_res_regist(&mut god);
        let mut context        = Context::new();
        let mut main           = Box::new(RGMain::new()) ;
        main.build(&parser,&god) ;
        debug!("main: {:?}", main) ;
        let main2 : ResBox = main ;
        main2.conf(&mut context) ;
        //main.start(&mut context) ;
        //main.start();
    }
}
