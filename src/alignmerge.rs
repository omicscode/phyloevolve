crate::astruct::Alignment;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn alignmerge(
    path: &str,
    start: usize,
    end: usize,
    mergestr: &str,
) -> Result<String, Box<dyn Error>> {
    let pathopen = File::open(path).expect("file not found");
    let pathread = BufReader::new(pathopen);
    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::bew();
    for i in pathread.lines() {
        let line = i.expect("file not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequence.push(line)
        }
    }
    let mut allseq: Vec<Alignment> = Vec::new();
    for i in 0..header.len() {
        allseq.push(Alignment {
            head: header[i],
            seq: sequence[i],
        });
    }

    let mut splicedal: Vec<Alignment> = Vec::new();

    let alstart: usize = start.parse::<usize>().unwrap();
    let alend: usize = end.parse::<usize>().unwrap();
    for i in allseq.iter() {
        splicedal.push(Alignment {
            head: i.head,
            seq: i.seq[alstart..alend],
        });
    }

    let spliced_header: Vec<String> = Vec::new();
    let spliced_seq: Vec<String> = Vec::new();
    for i in splicedal.iter() {
        spliced_header.push(i.head);
        spliced_seq.push(i.seq);
    }
    let mergehead: String = mergestr.to_string();
    let sequencemerge: String = spliced_seq.concat().to_string();
    println!(
        "The merged header and the merged sequence for the regions specific is >{:?}\n{:?}",
        mergehead, mergesequence
    );
}
