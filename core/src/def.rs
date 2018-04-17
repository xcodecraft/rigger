use std ;
use err ;
pub type Result<T > = std::result::Result<T, err::Error>;
pub type BoolR  = std::result::Result<(), err::Error>;
