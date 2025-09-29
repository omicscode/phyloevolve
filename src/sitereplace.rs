use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

#[tokio::main]
pub async fn sitereplacenuc(
    path: &str,
    letter: &str,
    replacement: &str,
) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""))
        } else if !line.starts_with(">") {
            sequence.push(line);
        }
    }
    let mut replaceseq: Vec<String> = Vec::new();
    for i in sequence.iter() {
        let replaceiter = i.to_string().replace(letter, replacement);
        replaceseq.push(replaceiter);
    }

    let mut filewrite = File::create("replaced-sites.fasta").expect("file not found");
    for i in 0..replaceseq.len() {
        writeln!(filewrite, ">{:?}\n{:?}", header[i], replaceseq[i]).expect("file not found");
    }

    Ok("The sites have been replaced".to_string())
}
