#[allow(unused_imports)]
use std::io::prelude::*;
use regex::{ Regex ,Captures };
use def::* ;
use std::env ;
use std ;
use std::convert::{From } ;

pub struct EExpress
{
    regex: Regex,
    data : StrMap,

}


impl EExpress
{
    pub fn new(data : StrMap ) -> EExpress
    {
        let regex = Regex::new(r"(\$\{([[:alnum:]_]+)\})").unwrap();
        EExpress{ regex, data  }
    }
    pub fn from_env() -> EExpress
    {
        let mut data = StrMap::new() ;
        for (key, value) in env::vars() {
            data.insert(key,value) ;
        }
        EExpress::new(data)
    }
    pub fn from_env_mix(map :StrMap) -> EExpress
    {
        let mut data = StrMap::new() ;
        for (key, value) in env::vars() {
            data.insert(key,value) ;
        }
        for (key,value) in map {
            data.insert(key,value) ;
        }
        //debug!("map data :{:?}", data);
        EExpress::new(data)
    }
    pub fn evar_val<'a>(&'a self, key : &str) -> Option<&'a String>
    {
         debug!("found key {:?}", key);
         self.data.get(key)
    }
    pub fn safe_evar_val(& self, key : &str) -> String
    {
         if let Some(val) = self.evar_val(key) 
         {
             return val.clone() ;
         }
         return format!("__NO[{}]__", key) ;
    }
    pub fn parse<T>(&self, content :T) -> String
        where UString: std::convert::From<T> 
    {
        let strc = UString::from(content);
        debug!("found {:?}", strc);
        let fun  =  | caps: &Captures| { format!("{}", self.safe_evar_val(&caps[2])) } ;
        self.regex.replace_all( strc.to_string().as_str() ,&fun).to_string()
    }
}

#[cfg(test)]
mod tests
{
    use super::* ;
    #[test]
    pub fn regex_verif()
    {
        let re      = Regex::new(r"(\$\{([[:alnum:]]+)\})").unwrap();
        let fun     =  | caps: &Captures| { format!("{}", &caps[2]) } ;
        let newc    = re.replace_all( "${HOME}/bin",  &fun) ;
        assert_eq!("HOME/bin",newc) ;
        let newc    = re.replace_all( "${HOME}/bin/${USER}",  &fun) ;
        assert_eq!("HOME/bin/USER",newc) ;
        let newc    = re.replace_all( "${HOME/bin",  &fun) ;
        assert_eq!("${HOME/bin",newc) ;

        let newc    = re.replace_all( "{HOME}/bin",  &fun) ;
        assert_eq!("{HOME}/bin",newc) ;
    }
    #[test]
    pub fn evar_verif()
    {
        let data = map!( 
            "HOME" => "/home/rigger",
            "USER" => "rigger"
            );
        
        let ex = EExpress::new(data);
        assert_eq!(ex.parse("${HOME}/bin"),String::from("/home/rigger/bin"));
        assert!(ex.parse("${HOME}/bin") != String::from("/home/rigger1/bin"));
        assert_eq!(ex.parse("${HOME}/${USER}/bin"),String::from("/home/rigger/rigger/bin"));
        assert_eq!(ex.parse("${HOME2}"),String::from("__NO[HOME2]__"));
        assert_eq!(ex.parse(format!("HOME2")),String::from("HOME2"));

        let content = format!("HOME2") ;
        assert_eq!(ex.parse(&content),String::from("HOME2"));
    }
    #[test]
    pub fn evar_verif2()
    {
        let data = map!( 
            "HOME"      => "/home/rigger",
            "USER"      => "rigger",
            "CUR_DIR"   => "/home",
            );
        let ex = EExpress::from_env_mix(data);
        assert_eq!(ex.parse("${HOME}/bin"),String::from("/home/rigger/bin"));
        assert_eq!(ex.parse("${CUR_DIR}/bin"),String::from("/home/bin"));
    }

}
