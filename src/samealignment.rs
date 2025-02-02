use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn samealignment(path: &str) -> Result<String, Box<dyn Error>> {
    let alignmentopen = File::open(path).expect("file not found");
    let alignmentread = BufReader::new(alignmentopen);

    let mut alignmenthash_header: HashSet<String> = HashSet::new();
    let mut alignmenthash_seq: HashSet<String> = HashSet::new();

    for i in alignmentread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            alignmenthash_header.insert(line.replace(">", ""));
        }
        if !line.starts_with(">") {
            alignmenthash_seq.insert(line);
        }
    }

    let mut uniquealignment = File::create("filtered-alignment.fasta").expect("file not found");
    for i in 0..alignmenthash_header.len() {
        writeln!(
            uniquealignment,
            ">{:?}\n{:?}",
            alignmenthash_header[i], alignmenthash_seq[i]
        );
    }

    Ok(
        "The filtered alignment with the same headers and the sequence check have been remvoed"
            .to_string(),
    )
}
