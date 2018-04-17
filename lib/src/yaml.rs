
extern crate yaml_rust;
use self::yaml_rust::{YamlLoader };


#[cfg(test)]
mod tests
{
    use super::* ;
    use std::io::prelude::*;
    use std::fs::File ;
    use pretty_env_logger ;
    struct StubParser
    {
    }
    #[test]
    pub fn load_yaml()
    {
        pretty_env_logger::init();
        let mut file = File::open("./material/run.yaml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
        debug!("data {:?}",docs) ;
    }
}
