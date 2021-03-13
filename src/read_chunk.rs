use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_in_chunks() -> std::io::Result<()> {
    const CAP: usize = 1024 * 128;
    let file = File::open("my.file")?;
    let mut reader = BufReader::with_capacity(CAP, file);

    loop {
        let length = {
            let buffer = reader.fill_buf()?;
            // do stuff with buffer here
            buffer.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length);
    }

    Ok(())
}
