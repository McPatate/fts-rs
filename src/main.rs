use log::info;
use quick_xml::de::{from_str, DeError};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug, Deserialize, PartialEq)]
struct Doc {
    title: String,
    url: String,
    r#abstract: String,
}

impl Clone for Doc {
    fn clone(&self) -> Doc {
        Doc {
            title: self.title.clone(),
            url: self.url.clone(),
            r#abstract: self.r#abstract.clone(),
        }
    }
}

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

fn naive_search(docs: &Vec<Doc>, term: &str) -> Vec<Doc> {
    let mut search_res: Vec<Doc> = Vec::with_capacity(docs.len() / 10);
    for doc in docs {
        if doc.r#abstract.contains(term) {
            search_res.push(doc.clone());
        }
    }
    search_res
}

fn tokenizer(phrase: String) -> Vec<String> {
    phrase
        .split(|c: char| !c.is_alphanumeric())
        .map(|s| s.to_string())
        .collect()
}

fn build_inverted_index() -> () {}

fn main() {
    env_logger::init();
    info!("loading wiki corpus");
    let xml = match load_corpus() {
        Ok(x) => x,
        Err(_) => panic!("Couldn't load wiki corpus"),
    };
    info!("parsing documents");
    let docs = match parse_documents(&xml) {
        Ok(d) => d,
        Err(e) => panic!("couldn't parse docs : {}", e),
    };
    info!("{} docs", docs.len());
    let search_term = "cat";
    info!("searching for {}", search_term);
    let inst = Instant::now();
    let search_res = naive_search(&docs, search_term);
    info!(
        "naively found {} matching {} in {} ms",
        search_res.len(),
        search_term,
        inst.elapsed().as_millis()
    );
}

// #[cfg(test)]
// mod search_tests {
//     use super::*;
//
//     #[bench]
//     fn naive_search_test() {
//         let xml = match load_corpus() {
//             Ok(x) => x,
//             Err(_) => panic!("Couldn't load wiki corpus"),
//         };
//         let docs = match parse_documents(&xml) {
//             Ok(d) => d,
//             Err(e) => panic!("couldn't parse docs : {}", e),
//         };
//         let search_term = "cat";
//         let search_res = naive_search(&docs, search_term);
//     }
// }
