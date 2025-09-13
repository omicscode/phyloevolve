use colored::Colorize;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-4
allows for the visualization of the spliced alignment
*/

#[tokio::main]
pub async fn splicedalignment(path: &str, start: usize, end: usize) {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct Embedded {
        header: String,
        sequence: String,
    }

    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(&fileopen);
    let mut embedded_hold: Vec<Embedded> = Vec::new();
    let mut hold_header: Vec<String> = Vec::new();
    let mut hold_sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            hold_header.push(line);
        } else {
            hold_sequence.push(line);
        }
    }
    for i in 0..hold_header.len() {
        embedded_hold.push(Embedded {
            header: hold_header[i].clone(),
            sequence: hold_sequence[i].clone(),
        })
    }
    let mut finalholdseq_multivector = Vec::new();
    let mut finalholdid_multivector: Vec<String> = Vec::new();
    for i in 0..hold_header.len() {
        let mut intermediatehold = Vec::new();
        for j in hold_sequence[i].chars() {
            intermediatehold.push(j);
        }
        finalholdseq_multivector.push(intermediatehold);
        finalholdid_multivector.push(hold_header[i].clone());
    }

    let mut addclipped: Vec<_> = Vec::new();
    for i in finalholdseq_multivector.iter_mut() {
        let clipped_alignment: Vec<_> = i[start..end].to_vec();
        addclipped.push(clipped_alignment);
    }

    for i in 0..addclipped.len() {
        for j in 0..addclipped[0].len() {
            if addclipped[i][j].to_string() == "A" {
                print!("{}", addclipped[i][j].to_string().on_yellow().bold())
            } else if addclipped[i][j].to_string() == "T" {
                print!("{}", addclipped[i][j].to_string().on_red().bold())
            } else if addclipped[i][j].to_string() == "C" {
                print!("{}", addclipped[i][j].to_string().on_green().bold())
            } else if addclipped[i][j].to_string() == "-" {
                print!("{}", addclipped[i][j].to_string().on_purple().bold())
            } else {
                continue;
            }
        }
        println!();
    }
}
