use std; 
use toml ;


#[derive( Debug)]
pub enum Error
{
    Runtime(String),
    Logic(LogicErr),
    Format(std::str::Utf8Error),
    Io(std::io::Error),
    Timeout(String),
    Config(toml::de::Error) ,
    Normal,
    Non,
}
#[derive( Debug)]
pub enum LogicErr
{
    Unexpect(String),
}

impl std::convert::From<toml::de::Error> for Error{
    fn from(error: toml::de::Error) -> Self {
        Error::Config(error)
    }
}


impl std::convert::From<String> for Error{
    fn from(error: String) -> Self {
        Error::Runtime(error)
    }
}
impl std::convert::From<&'static str> for Error{
    fn from(error: &str) -> Self {
        Error::Runtime(String::from(error))
    }
}
impl std::convert::From<std::str::Utf8Error> for Error
{
    fn from(error: std::str::Utf8Error) -> Self {
        Error::Format(error)
    }

}
impl std::convert::From<std::io::Error> for Error
{
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }

}
