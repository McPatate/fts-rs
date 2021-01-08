use log::info;
use quick_xml::de::{from_str, DeError};
use rust_stemmers::{Algorithm, Stemmer};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

const STOPWORDS: [&str; 127] = [
    "i",
    "me",
    "my",
    "myself",
    "we",
    "our",
    "ours",
    "ourselves",
    "you",
    "your",
    "yours",
    "yourself",
    "yourselves",
    "he",
    "him",
    "his",
    "himself",
    "she",
    "her",
    "hers",
    "herself",
    "it",
    "its",
    "itself",
    "they",
    "them",
    "their",
    "theirs",
    "themselves",
    "what",
    "which",
    "who",
    "whom",
    "this",
    "that",
    "these",
    "those",
    "am",
    "is",
    "are",
    "was",
    "were",
    "be",
    "been",
    "being",
    "have",
    "has",
    "had",
    "having",
    "do",
    "does",
    "did",
    "doing",
    "a",
    "an",
    "the",
    "and",
    "but",
    "if",
    "or",
    "because",
    "as",
    "until",
    "while",
    "of",
    "at",
    "by",
    "for",
    "with",
    "about",
    "against",
    "between",
    "into",
    "through",
    "during",
    "before",
    "after",
    "above",
    "below",
    "to",
    "from",
    "up",
    "down",
    "in",
    "out",
    "on",
    "off",
    "over",
    "under",
    "again",
    "further",
    "then",
    "once",
    "here",
    "there",
    "when",
    "where",
    "why",
    "how",
    "all",
    "any",
    "both",
    "each",
    "few",
    "more",
    "most",
    "other",
    "some",
    "such",
    "no",
    "nor",
    "not",
    "only",
    "own",
    "same",
    "so",
    "than",
    "too",
    "very",
    "s",
    "t",
    "can",
    "will",
    "just",
    "don",
    "should",
    "now",
];

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
        .split_terminator(|c: char| !c.is_alphanumeric())
        .map(|s| s.to_string())
        .collect()
}

fn lowercase_filter(tokens: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(tokens.len());
    for token in tokens {
        res.push(token.to_lowercase());
    }
    res
}

fn stopwords_filter(tokens: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(tokens.len());
    for token in tokens {
        if !STOPWORDS.contains(&&token[..]) {
            res.push(token.to_string());
        }
    }
    res
}

fn stemming_filter(stemmer: Stemmer, tokens: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(tokens.len());
    for token in tokens {
        res.push(stemmer.stem(&token[..]).to_string());
    }
    res
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

#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    fn tokenizer_test() {
        let tokens = vec!["Hello".to_string(), "world".to_string()];

        assert_eq!(tokenizer("Hello world!".to_string()), tokens);
    }

    #[test]
    fn lowercase_filter_test() {
        let lowered: Vec<String> = vec!["hello".to_string(), "world".to_string()];
        let upper: Vec<String> = vec!["Hello".to_string(), "world".to_string()];

        assert_eq!(lowercase_filter(upper), lowered);
    }

    #[test]
    fn stopwords_filter_test() {
        let filtered: Vec<String> = vec!["Hello".to_string(), "world".to_string()];
        let phrase: Vec<String> = vec![
            "Hello".to_string(),
            "to".to_string(),
            "the".to_string(),
            "world".to_string(),
        ];

        assert_eq!(stopwords_filter(phrase), filtered);
    }

    #[test]
    fn stemming_filter_test() {
        let en_stem: Stemmer = Stemmer::create(Algorithm::English);
        let filtered: Vec<String> = vec![
            "Hello".to_string(),
            "borrow".to_string(),
            "world".to_string(),
        ];
        let phrase: Vec<String> = vec![
            "Hello".to_string(),
            "borrowed".to_string(),
            "worldly".to_string(),
        ];

        assert_eq!(stemming_filter(en_stem, phrase), filtered);
    }

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
}
