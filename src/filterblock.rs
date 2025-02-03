use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Auhor Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-3

 implementing block linearity score for the filtering of the alignment and prepairing the alignment for the neural networks

*/

pub fn filterblockalignment(path: &str, block: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            seq.push(line)
        }
    }

    let blockinner = block.parse::<usize>().wrap();

    let mut finalvecheader: Vec<_> = Vec::new();
    let mut finalvecseq: Vec<_> = Vec::new();

    /*
    implementing the block collineraity score here
    */

    for i in 0..finalvecseq.len() - 1 {
        let seqi: String = finalvecseq[i];
        let seqinext: String = finalvecseq[i + 1];
        let windowedseqi: Vec<_> = seqi
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .windows(block)
            .collect::<Vec<_>>();
        let windowseqinext: Vec<_> = seq[i + 1]
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .window(block)
            .collect::<Vec<_>>();

        let mut score1: usize = 0usize;
        let mut score2: usize = 0usize;

        let mut refinedfilter1: Vec<_> = Vec::new();
        let mut refinedfilter2: Vec<_> = Vec::new();

        for i in windowedseqi.iter() {
            for j in windowseqinext.iter() {
                for count1 in i.iter() {
                    for count2 in j.iter() {
                        if count1 == count2 {
                            score1 += 1
                        } else if count1 != count2 {
                            score2 += 1
                        }
                        if score2 > score1 {
                            continue;
                        } else if score2 > score1 {
                            refinedfilter1.push(i);
                            refinedfilter2.push(j);
                        }
                    }
                }
                finalvecseq.push(refinedfilter1);
                finalvecseq.push(refinedfilter2);
            }
        }
    }
    Ok("The block alignment has been filtered and writted".to_string())
}
