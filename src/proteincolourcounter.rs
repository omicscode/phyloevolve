use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
/*
  Author Gaurav Sablok
  SLB Potsdam
  Date: 2025-2-8

  protein colour coded heatmaps


*/

pub fn proteomecolor(path: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    let _proteinarr: Vec<String> = vec![
        "ALA".to_string(),
        "ARG".to_string(),
        "ASN".to_string(),
        "ASP".to_string(),
        "CYS".to_string(),
        "GLU".to_string(),
        "GLN".to_string(),
        "GLY".to_string(),
        "HIS".to_string(),
        "ILE".to_string(),
        "LEU".to_string(),
        "LYS".to_string(),
        "MET".to_string(),
        "PHE".to_string(),
        "PRO".to_string(),
        "SER".to_string(),
        "THR".to_string(),
        "TRP".to_string(),
        "TYR".to_string(),
        "VAL".to_string(),
    ];
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequence.push(line);
        }
    }

    let mut finalcount: Vec<(String, Vec<(String, usize)>)> = Vec::new();
    for i in 0..sequence.len() {
        let mut ausize: usize = 0usize;
        let mut rusize: usize = 0usize;
        let mut nusize: usize = 0usize;
        let mut dusize: usize = 0usize;
        let mut cusize: usize = 0usize;
        let mut eusize: usize = 0usize;
        let mut qusize: usize = 0usize;
        let mut gusize: usize = 0usize;
        let mut husize: usize = 0usize;
        let mut iusize: usize = 0usize;
        let mut lusize: usize = 0usize;
        let mut kusize: usize = 0usize;
        let mut musize: usize = 0usize;
        let mut fusize: usize = 0usize;
        let mut pusize: usize = 0usize;
        let mut susize: usize = 0usize;
        let mut tusize: usize = 0usize;
        let mut wusize: usize = 0usize;
        let mut yusize: usize = 0usize;
        let mut vusize: usize = 0usize;
        let ihold: Vec<String> = sequence[i].chars().map(String::from).collect::<Vec<_>>();
        let mut iholdtuple: Vec<(String, usize)> = Vec::new();
        for i in ihold.iter() {
            if i == "A" {
                ausize += 1usize;
            } else if i == "R" {
                rusize += 1usize;
            } else if i == "N" {
                nusize += 1usize;
            } else if i == "D" {
                dusize += 1usize;
            } else if i == "C" {
                cusize += 1usize;
            } else if i == "E" {
                eusize += 1usize;
            } else if i == "Q" {
                qusize += 1usize;
            } else if i == "G" {
                gusize += 1usize;
            } else if i == "H" {
                husize += 1usize;
            } else if i == "I" {
                iusize += 1usize;
            } else if i == "L" {
                lusize += 1usize;
            } else if i == "K" {
                kusize += 1usize;
            } else if i == "M" {
                musize += 1usize;
            } else if i == "F" {
                fusize += 1usize;
            } else if i == "P" {
                pusize += 1usize;
            } else if i == "S" {
                susize += 1usize;
            } else if i == "T" {
                tusize += 1usize;
            } else if i == "W" {
                wusize += 1usize;
            } else if i == "Y" {
                yusize += 1usize;
            } else if i == "V" {
                vusize += 1usize;
            } else {
                continue;
            }
        }

        iholdtuple.push(("A".to_string(), ausize));
        iholdtuple.push(("R".to_string(), rusize));
        iholdtuple.push(("N".to_string(), nusize));
        iholdtuple.push(("D".to_string(), dusize));
        iholdtuple.push(("C".to_string(), cusize));
        iholdtuple.push(("E".to_string(), eusize));
        iholdtuple.push(("Q".to_string(), qusize));
        iholdtuple.push(("G".to_string(), gusize));
        iholdtuple.push(("H".to_string(), husize));
        iholdtuple.push(("I".to_string(), iusize));
        iholdtuple.push(("L".to_string(), lusize));
        iholdtuple.push(("K".to_string(), kusize));
        iholdtuple.push(("M".to_string(), musize));
        iholdtuple.push(("F".to_string(), fusize));
        iholdtuple.push(("P".to_string(), pusize));
        iholdtuple.push(("S".to_string(), susize));
        iholdtuple.push(("T".to_string(), tusize));
        iholdtuple.push(("W".to_string(), wusize));
        iholdtuple.push(("Y".to_string(), yusize));
        iholdtuple.push(("V".to_string(), vusize));
        finalcount.push((sequence[i].clone(), iholdtuple));
    }

    let mut med_a: Vec<usize> = Vec::new();
    let mut med_r: Vec<usize> = Vec::new();
    let mut med_n: Vec<usize> = Vec::new();
    let mut med_d: Vec<usize> = Vec::new();
    let mut med_c: Vec<usize> = Vec::new();
    let mut med_e: Vec<usize> = Vec::new();
    let mut med_q: Vec<usize> = Vec::new();
    let mut med_g: Vec<usize> = Vec::new();
    let mut med_h: Vec<usize> = Vec::new();
    let mut med_i: Vec<usize> = Vec::new();
    let mut med_l: Vec<usize> = Vec::new();
    let mut med_k: Vec<usize> = Vec::new();
    let mut med_m: Vec<usize> = Vec::new();
    let mut med_f: Vec<usize> = Vec::new();
    let mut med_p: Vec<usize> = Vec::new();
    let mut med_s: Vec<usize> = Vec::new();
    let mut med_t: Vec<usize> = Vec::new();
    let mut med_w: Vec<usize> = Vec::new();
    let mut med_y: Vec<usize> = Vec::new();
    let mut med_v: Vec<usize> = Vec::new();

    for i in finalcount.iter(){
        for j in i.1.iter(){
            if j.0 == "A" {
                med_a.push(j.1)
            } else if j.0 == "R" {
                med_r.push(j.1);
            } else if j.0 == "N" {
                med_n.push(j.1);
            } else if j.0 == "D" {
                med_d.push(j.1);
            } else if j.0 == "C" {
                med_c.push(j.1);
            } else if j.0 == "E" {
                med_e.push(j.1);
            } else if j.0 == "Q" {
                med_q.push(j.1);
            } else if j.0 == "G" {
                med_g.push(j.1)
            } else if j.0 == "H" {
                med_h.push(j.1);
            } else if j.0 == "I" {
                med_i.push(j.1);
            } else if j.0 == "L" {
                med_l.push(j.1);
            } else if j.0 == "K" {
                med_k.push(j.1);
            } else if j.0 == "M" {
                med_m.push(j.1);
            } else if j.0 == "F" {
                med_f.push(j.1);
            } else if j.0 == "E" {
            med_e.push(j.1);
        } else if j.0 == "Q" {
            med_f.push(j.1);
        } else if j.0 == "G" {
            med_g.push(j.1);
        } else if j.0 == "H" {
            med_h.push(j.1);
        } else if j.0 == "I" {
            med_i.push(j.1);
        } else if j.0 == "L" {
            med_l.push(j.1);
        } else if j.0 == "K" {
            med_k.push(j.1);
        } else if j.0 == "M" {
            med_m.push(j.1);
        } else if j.0 == "F" {
            med_f.push(j.1);
        } else if j.0 == "P" {
            med_p.push(j.1);
        } else if j.0 == "S"{
            med_s.push(j.1);
        } else if j.0 == "T"{
            med_t.push(j.1);
        } else if j.0 == "W"{
            med_w.push(j.1);
        } else if j.0 == "Y" {
            med_y.push(j.1);
        } else if j.0 == "V"{
            med_v.push(j.1);
        }
     }
        
    for i in 0..finalcount.len() {
        let istring: String = finalcount[i].0.clone();
        let ivec: Vec<(String, usize)> = finalcount[i].1.clone();
        for j in ivec.iter(){
            print!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", istring, "A".to_string(), "R".to_string(), "N".to_string(), "D".to_string(), "C".to_string(), "E".to_string(), "Q".to_string(), "G".to_string(), "H".to_string(), "I".to_string(), "L".to_string(), "K".to_string(), "M".to_string(), "F".to_string(), "P".to_string(), "S".to_string(), "T".to_string(), "W".to_string(), "Y".to_string())
            if j.1 > 
        }
    }

    Ok("proteome stats have been written".to_string())
}
