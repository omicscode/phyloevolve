use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*

 Author Gaurav Sablok
 SLB Potsdam
 Date 2025-2-4


  alignment site replace with the specific nucleotide
*/

pub fn sitereplacenuc(
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
    for i in replaceseq.iter() {
        let replaceiter = i.to_string().replace(letter, replacement);
        replaceseq.push(replaceiter);
    }

    Ok("The sites have been replaced".to_string())
}
