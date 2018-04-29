
extern crate yaml_rust;
use std::result::Result ;
use std::fs::File ;
#[allow(unused_imports)]
use std::io::prelude::* ;
use self::yaml_rust::{YamlLoader,ScanError,Yaml };

#[allow(dead_code)]
pub type YamlVec =  Vec<Yaml> ;
#[allow(dead_code)]
struct ConfParser
{
}
impl ConfParser 
{
    #[allow(dead_code)]
    pub fn parse_file(conf_file : &str) ->Result<YamlVec, ScanError>
    {
        debug!("parse file:{} ", conf_file);
        let mut file = File::open(conf_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        YamlLoader::load_from_str(contents.as_str())
    }
}

#[cfg(test)]
mod tests
{
    use super::* ;
    #[allow(unused_imports)]
    use pretty_env_logger ;
    use self::yaml_rust::{yaml };
    #[test]
    pub fn load_yaml()
    {
        pretty_env_logger::init();
        let docs = ConfParser::parse_file("./material/run_mid.yaml").unwrap() ;
        for doc in &docs {
            println!("---");
            dump_node(doc, 0);
        }
    }

    fn print_indent(indent: usize) {
        for _ in 0..indent {
            print!("    ");
        }
    }

    fn dump_node(doc: &yaml::Yaml, indent: usize) {
        match doc {
            &yaml::Yaml::Array(ref v) => {
                for x in v {
                    dump_node(x, indent + 1);
                }
            },
            &yaml::Yaml::Hash(ref h) => {
                for (k, v) in h {
                    print_indent(indent);
                    println!("{:?}:", k);
                    dump_node(v, indent + 1);
                }
            },
            _ => {
                print_indent(indent);
                println!("{:?}", doc);
            }
        }
    }
}
