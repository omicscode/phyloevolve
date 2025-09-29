use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

#[tokio::main]
pub async fn filtersiteremoval(path: &str, base: &str) -> Result<String, Box<dyn Error>> {
    let pathopen = File::open(path).expect("file not found");
    let pathread = BufReader::new(pathopen);

    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();
    let mut headerfilter: Vec<String> = Vec::new();
    let mut seqfilter: Vec<String> = Vec::new();

    for i in pathread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""))
        } else if !line.starts_with(">") {
            seq.push(line)
        }
    }

    for i in 0..seq.len() {
        let headerinter = header[i].clone();
        let seqcap: Vec<_> = seq[i]
            .chars()
            .filter(|x| *x.to_string() != base.to_string())
            .collect::<Vec<_>>();
        let filteredstring = seqcap
            .into_iter()
            .map(|x| String::from(x))
            .collect::<Vec<_>>()
            .join("")
            .to_string();
        headerfilter.push(headerinter);
        seqfilter.push(filteredstring);
    }

    let mut filewrite = File::create("filtered-sites.fasta").expect("file not found");
    for i in 0..headerfilter.len() {
        writeln!(filewrite, ">{:?}\n{:?}", headerfilter[i], seqfilter[i]).expect("line not found");
    }

    Ok("the filtered sites have been written".to_string())
}
