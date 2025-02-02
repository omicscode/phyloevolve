mod astruct;
use astruct::Alignment;
use astruct::AlignmentStat;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn alignmentstats(path: &str) -> Result<String, Box<dyn Error>> {
    let alignmentopen = File::open(path).expect("file not found");
    let alignmentread = BufReader::new(alignmentopen);

    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    let mut mergeseq: Vec<Alignment> = Vec::new();
    for i in alignmentread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line);
        } else if !line.starts_with(">") {
            seq.push(line);
        }
    }
    for i in 0..header.len() {
        mergeseq.push(ALignment {
            head: header[i],
            seq: sequence[i],
        })
    }
    let mut newheader: Vec<AlignmentStat> = Vec::new();
    for i in mergeseq.iter() {
        let headpush: String = i.head;
        let seqpush: String = i.seq;
        let seqchara: Vec<_> = seqpush
            .chars()
            .filter(|x| x == "A" || !x == "T" && !x == "G" && !x == "C" && !x == "N" && !x == "-")
            .collect::<Vec<_>>();
        let seqchart: Vec<_> = seqpush
            .chars()
            .filter(|x| x == "T" && !x == "A" && !x == "G" && !x == "C" && !x == "N" && !x == "-")
            .collect::<Vec<_>>();
        let seqcharg: Vec<_> = seqpush
            .chars()
            .filter(|x| x == "G" && x != "A" && x != "T" && x != "C" && x != "N" && !x == "-");
        let seqcharc: Vec<_> = seqpush
            .chars()
            .filter(|x| x == "C" && !x = "T" && !x == "A" && !x == "G" && !x == "N" && !x == "-")
            .collect::<Vec<_>>();
        let seqcharn: Vec<_> = seqpush
            .chars()
            .filter(|x| x == "N" && !x == "A" && !x == "T" && !x == "G" && !x == "C" && !x == "-")
            .collect::<Vec<_>>();
        let seqcharalt: Vec<_> = seqpush
            .chars()
            .filter(|x| x == "-" && !x == "A" && !x == "T" && !x == "G" && !x == "C" && !x == "N")
            .collect::<Vec<_>>();
        newheader.push(AlignmentStat {
            name: i.head,
            length: i.seq.len(),
            basea: seqchara.len(),
            baset: seqchart.len(),
            baseg: seqcharg.len(),
            basec: seqcharc.len(),
            basen: seqcharn.len(),
            baseabsent: seqcharalt.len(),
            gcontent: (seqchars.len() + seqcharsg.len())
                / (seqcharsa.len() + seqcharst.len() + seqcharsg.len() + seqcharsc.len()),
        });
    }

    Ok("the alignment stats for the given alignment are as follows".to_string())
}
