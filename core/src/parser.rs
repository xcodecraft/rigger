use def::* ;
#[derive(Debug,Clone)]
pub enum  RgvType{
    Vars,
    Env,
    System,
    Project,
    //Modul,
    Res(String),
}
pub fn res_of( key :&str) -> RgvType
{
    RgvType::Res(String::from(key)) 

}

#[derive(Debug,Clone)]
pub enum  ParseState
{
    In,
    End,
}

#[derive(Debug,Clone)]
pub struct Node
{
    pub rkey : RgvType,
    pub data : StrMap,
}
#[derive(Debug,Clone)]
pub struct ParseResult
{
    pub state : ParseState,
    pub node  : Option<Node>,
}
impl ParseResult 
{
    pub fn inn(  rkey: RgvType, data: StrMap )->ParseResult
    {
        ParseResult { state: ParseState::In, node : Some( Node {rkey, data  } ) }
    }
    pub fn end() -> ParseResult
    {
        ParseResult{ state: ParseState::End , node : None }
    }

}
pub trait Parser
{
    fn  next(&self) -> Option<ParseResult> ;
}
pub type ParserBox = Box<Parser> ;


use std::cell::RefCell ;
pub type RgDataVec  = Vec<ParseResult> ;
pub struct StubParser
{
    data : RefCell<RgDataVec> ,
}
impl StubParser
{
    pub fn new( mut data : RgDataVec) -> StubParser
    {
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
