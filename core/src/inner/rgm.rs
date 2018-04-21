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

impl SellDesp for RGMain
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


impl CallPlugin for RGMain
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
    type RgDataVec  = Vec<ParseResult> ;
    struct StubParser
    {
        data : RefCell<RgDataVec> ,
    }
    impl StubParser
    {
        pub fn new() -> StubParser
        {
            let env = map!( "_name" => "dev" ) ;
            let mut data = vec![
                ParseResult::new(ParseState::In  , RgvType::Env    , map!( "_name" => "dev" ))  ,
                ParseResult::new(ParseState::In  , RgvType::Vars   , map!( "x"     => "256"   , "y" => "24")) ,
                ParseResult::end() ,
                ParseResult::new(ParseState::In  , RgvType::System , map!( "_name" => "api" ) ) ,
                ParseResult::new(ParseState::In  , RgvType::Vars   , map!( "x"     => "256"        , "y" => "24")) ,
                ParseResult::end(),
            ];
            data.reverse();

            StubParser{ data : RefCell::new(data)}
        }
    }
    impl  Parser for  StubParser
    {
        fn  next(&self) -> Option<ParseResult>
        {
            let mut data = self.data.borrow_mut() ;
            data.pop()
        }
    }

    #[test]
    fn use_it()
    {
        pretty_env_logger::init();
        let parser : ParserBox = Box::new(StubParser::new()) ;
        let mut context    = Context::new();
        let mut main   = RGMain::new() ;
        main.build(&parser) ;
        debug!("main: {:?}", main) ;
        main.conf(&mut context) ;
        //main.start();
    }
}
