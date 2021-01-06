use quick_xml::de::{from_str, DeError};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Deserialize, PartialEq)]
struct Doc {
    title: String,
    url: String,
    r#abstract: String,
}

// impl Doc {
//     fn new() -> Doc {
//         Doc {
//             title: "".to_string(),
//             url: "".to_string(),
//             text: "".to_string(),
//             id: 0,
//         }
//     }
// }

fn load_corpus() -> std::io::Result<String> {
    let file = File::open("enwiki-latest-abstract1.xml")?;
    let mut br = BufReader::new(file);
    let mut xml = String::new();
    br.read_to_string(&mut xml)?;
    Ok(xml)
}

fn parse_documents(xml: &str) -> Result<Vec<Doc>, DeError> {
    let docs = from_str::<Vec<Doc>>(xml)?;
    Ok(docs)
}

fn main() {
    let xml = match load_corpus() {
        Ok(x) => x,
        Err(_) => panic!("Couldn't load wiki corpus"),
    };
    let docs = match parse_documents(&xml) {
        Ok(d) => d,
        Err(e) => panic!("couldn't parse docs : {}", e),
    };
    println!("{:?}", docs);
}
