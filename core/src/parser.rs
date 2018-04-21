use def::* ;
#[derive(Debug,Clone)]
pub enum  RgvType{
    Vars,
    Env,
    System,
    Project,
    //Modul,
    Res,
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

