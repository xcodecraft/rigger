
use model::* ;
use def::* ;
use res::* ;
use super::* ;
#[derive(Debug)]
pub struct RGMain
{
    resvec : ResVec ,
}

impl RGMain 
{
    pub fn new( ) ->RGMain
    {
        RGMain{ resvec : ResVec::new() }
    }
}



#[cfg(test)]
mod tests
{
    use super::* ;
    use model::Parser ;
    use std::cell::RefCell;
    use pretty_env_logger ;
    type RgDataVec  = Vec<(RgvType, StrMap)> ;
    struct StubParser
    {
        data : RefCell<RgDataVec> ,
    }
    impl StubParser
    {
        pub fn new() -> StubParser
        {
            let env = map!( "_name" => "dev" ) ;
            let data = vec![
                 (RgvType::Vars, map!( "x" => "256", "y" => "24")),
                 (RgvType::Env, map!( "_name" => "dev" ) ),
            ];

            StubParser{ data : RefCell::new(data)}
        }
    }
    impl  Parser for  StubParser
    {
        fn  next(&self) -> Option<(RgvType,StrMap)>
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
        let mut main   = RGMain::new() ;
        main.build(&parser) ;
        debug!("main: {:?}", main) ;
        //main.conf() ;
        //main.start();
    }
}
