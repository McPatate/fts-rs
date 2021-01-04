use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn load_xml() -> std::io::Result<String> {
    let file = File::open("enwiki-latest-abstract1.xml")?;
    let mut br = BufReader::new(file);
    let mut xml = String::new();
    br.read_to_string(&mut xml)?;
    Ok(xml)
}

fn parse_documents(xml: &str) -> () {
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    let mut _count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"tag1" => println!(
                    "attributes values: {:?}",
                    e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                ),
                b"tag2" => _count += 1,
                _ => (),
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
}

fn main() {
    let xml = match load_xml() {
        Ok(x) => x,
        Err(_) => panic!("Couldn't load wiki corpus"),
    };
    parse_documents(&xml);
}
