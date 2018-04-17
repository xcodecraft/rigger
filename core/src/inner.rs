
pub struct Vars
{

}

pub struct Env
{

}
pub struct System
{

}

pub struct Project 
{

}
pub struct Modul
{

}

pub struct RGMain
{

}

#[cfg(test)]
mod tests
{
    struct StubParser
    {
        resvec : ResVec ,
    }
    impl StubParser
    {
        pub fn  new ()
        {
            let res = Vec! [
                Box::new(Env::new()),
                Box::new(Var::new()),
                Box::new(Var::new()),
                Box::new(Var::new()),
                Box::new(Sys::new()),
            ];
            StubParser { resvec : res } 
        }
        fn parse() -> ResBox ;
        {
        }
    }
    fn use()
    {
        let main = RGMain();
        loop {
            let robj = to_robj() ;
            main.regist(robj)
        }
        main.conf() ;
        main.start();
    }
}
