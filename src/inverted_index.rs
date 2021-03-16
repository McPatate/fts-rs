use crate::query_parser::QueryParser;
use crate::wiki::WikiDoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct InvertedIndex {
    idx: HashMap<String, Vec<usize>>,
    doc_count: usize,
}

impl InvertedIndex {
    pub fn open(&mut self, fp: &str) -> std::io::Result<()> {
        let file = File::open(fp)?;
        let br = BufReader::new(file);
        self.idx = serde_json::from_reader(br)?;
        Ok(())
    }

    pub fn save(&self, fp: &str) -> std::io::Result<()> {
        let file = File::create(fp)?;
        let mut bw = BufWriter::new(file);
        serde_json::to_writer(&mut bw, &self.idx)?;
        Ok(())
    }

    pub fn add_wiki_doc(&mut self, doc: &WikiDoc, doc_id: usize) {
        let tokens = InvertedIndex::tokenizer(doc.r#abstract.clone());
        let lowered_tokens = InvertedIndex::lowercase_filter(tokens);
        for lt in lowered_tokens {
            if self.idx.contains_key(&lt) {
                let postings = match self.idx.get_mut(&lt) {
                    Some(v) => v,
                    None => panic!("unintialized postings list"),
                };
                if !postings.contains(&doc_id) {
                    // TODO compare efficiency
                    postings.push(doc_id);
                    // InvertedIndex::sorted_insert(postings, doc_id);
                }
            } else {
                self.idx.insert(lt, vec![doc_id]);
            }
        }
        self.doc_count += 1;
    }

    pub fn new(doc_count: usize) -> InvertedIndex {
        InvertedIndex {
            idx: HashMap::new(),
            doc_count: doc_count,
        }
    }

    // here Option should be a Result in case query parsing errors
    pub fn search(&self, query: &str) -> Option<Vec<usize>> {
        let qp = QueryParser::new();
        let tokens = qp.to_postfix(query);
        let all_postings: Vec<usize> = (0..self.doc_count).collect();
        if tokens.is_none() {
            return None;
        }
        let tokens = tokens.unwrap();
        let mut stack: Vec<Vec<usize>> = Vec::with_capacity(tokens.len());
        for token in tokens {
            if !qp.operators.contains(&token) {
                match self.idx.get(&token) {
                    Some(pl) => stack.push(pl.to_vec()),
                    None => stack.push(Vec::new()),
                }
            } else if token == "NOT" {
                let pl = stack.pop().unwrap();
                stack.push(InvertedIndex::intersect_not(&all_postings, &pl));
            } else if token == "AND" {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(InvertedIndex::intersect(&l, &r));
            } else if token == "OR" {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(InvertedIndex::merge(&l, &r));
            }
        }
        stack.pop()
    }

    fn intersect_not(p1: &Vec<usize>, p2: &Vec<usize>) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < p1.len() && j < p2.len() {
            if p1[i] == p2[j] {
                i += 1;
                j += 1;
            } else if p1[i] < p2[j] {
                res.push(p1[i]);
                i += 1;
            } else {
                j += 1;
            }
        }
        while i < p1.len() {
            res.push(p1[i]);
            i += 1;
        }
        res
    }

    fn intersect(p1: &Vec<usize>, p2: &Vec<usize>) -> Vec<usize> {
        let mut res = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < p1.len() && j < p2.len() {
            if p1[i] == p2[j] {
                res.push(p1[i]);
                i += 1;
                j += 1;
            } else if p1[i] < p2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        res
    }

    fn merge(p1: &Vec<usize>, p2: &Vec<usize>) -> Vec<usize> {
        let mut res = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < p1.len() {
            res.push(p1[i]);
            i += 1;
        }
        while j < p2.len() {
            res.push(p2[j]);
            j += 1;
        }
        res.dedup();
        res
    }

    fn lowercase_filter(tokens: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = Vec::with_capacity(tokens.len());

        for token in tokens {
            res.push(token.to_lowercase());
        }
        res
    }

    fn tokenizer(phrase: String) -> Vec<String> {
        phrase
            .split_terminator(|c: char| !c.is_alphanumeric())
            .map(|s| s.to_string())
            .collect()
    }

    fn sorted_insert(p: &mut Vec<usize>, v: usize) {
        let mut low = 0;
        let mut high = p.len() - 1;
        let ipos;

        while low <= high {
            let mid = (high + low) / 2;
            if p[mid] < v {
                low = mid + 1
            } else if p[mid] > v {
                high = mid - 1
            }
        }
        if low > high {
            ipos = high + 1;
        } else {
            ipos = low
        }
        let upper_bound = v + 1;
        p.splice(ipos..ipos, v..upper_bound);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_search() {}
}
