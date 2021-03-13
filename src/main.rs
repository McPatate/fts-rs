mod inverted_index;
mod skiplist;
mod wiki;

use inverted_index::InvertedIndex;
use std::io::{stdin, stdout, Write};
use wiki::WikiDoc;

fn naive_search<'a>(query: &'a str, docs: &'a Vec<WikiDoc>) -> Vec<&'a WikiDoc> {
    let unwanted: &[_] = &[' ', '\t', '\n'];
    let trimmed = query.trim_matches(unwanted);
    let mut res: Vec<&WikiDoc> = Vec::new();
    for doc in docs {
        if doc.r#abstract.contains(&trimmed) {
            res.push(doc);
        }
    }
    res
}

fn main() {
    let docs = match wiki::parse_documents(
        "/Users/mc/Documents/boolean_retrieval/enwiki-latest-abstract1.xml",
    ) {
        Ok(d) => d,
        Err(e) => panic!("err : {}", e),
    };
    let mut ii = InvertedIndex::new(docs.len());
    match ii.open("inv_idx.json") {
        Ok(_) => println!("succesfully loaded inv idx - {} docs", docs.len()),
        Err(e) => panic!("err : {}", e),
    };
    // for i in 0..docs.len() {
    //     ii.add_wiki_doc(&docs[i], i);
    // }
    // match ii.save("/Users/mc/Documents/boolean_retrieval/inv_idx.json") {
    //     Ok(_) => println!("saved file succesfully"),
    //     Err(e) => panic!("err : {}", e),
    // };
    loop {
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Invalid string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        // let res = naive_search(&s, &docs);
        // for r in &res {
        //     println!("\n---- {} ----\n{}\n", r.title, r.r#abstract);
        // }
        // println!("{} hits", res.len());
        match ii.search(&s) {
            Some(results) => {
                for r in &results {
                    println!("\n---- {} ----\n{}\n", docs[*r].title, docs[*r].r#abstract);
                }
                println!("{} hits\n", results.len());
            }
            None => println!("Nothing found / Invalid query"),
        };
    }
}
