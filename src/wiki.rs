use quick_xml::de::{from_str, DeError};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq)]
pub struct WikiDoc {
    #[serde(skip)]
    pub id: String,
    pub title: String,
    url: String,
    pub r#abstract: String,
}

fn load_corpus(fp: &str) -> std::io::Result<String> {
    let file = File::open(fp)?;
    let mut br = BufReader::new(file);
    let mut xml = String::new();
    br.read_to_string(&mut xml)?;
    Ok(xml)
}

pub fn parse_documents(fp: &str) -> Result<Vec<WikiDoc>, DeError> {
    let xml = match load_corpus(fp) {
        Ok(s) => s,
        Err(e) => panic!("err : {}", e),
    };
    let mut docs = from_str::<Vec<WikiDoc>>(&xml)?;
    for doc in &mut docs {
        doc.id = Uuid::new_v4().to_string();
    }
    Ok(docs)
}
