
use model::* ;
use def::* ;
use res::* ;
use super::* ;
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

impl InnerContainer for RGMain {

    fn resvec_hold<'a>(&'a mut self) ->&'a mut  ResVec 
    {
        &mut self.resvec
    }
}


#[cfg(test)]
mod tests
{
    use super::* ;
    use model::Parser ;
    use std::cell::RefCell;


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
                // (RgvType::Env, map!( "_name" => "dev" ) ),
                // (RgvType::Vars, map!( "x" => "256", "y" => "24")),
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
        let parser = ParserBox::new(StubParser::new()) ;
        let main   = RGMain::new() ;
        main.build(&parser) ;
        //main.conf() ;
        //main.start();
    }
}
