
use super::* ;
impl InnerContainer for Env {

    fn resvec_hold<'a>(&'a self) ->&'a ResVec 
    {
        &self.resvec
    }
}
impl InnerContainer for Project {

    fn resvec_hold<'a>(&'a self) ->&'a ResVec 
    {
        &self.resvec
    }
}

impl InnerContainer for RGMain {

    fn resvec_hold<'a>(&'a self) ->&'a ResVec 
    {
        &self.resvec
    }
}

impl InnerContainer for System {

    fn resvec_hold<'a>(&'a self) ->&'a ResVec 
    {
        &self.resvec
    }
}
