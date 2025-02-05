use crate::filesearch::read;
use std::error::Error;

/*

 Author Gaurav Sablok
 SLB Potsdam
 Date 2025-2-5

 Motif search and placeholder across the alignment.

*/

pub fn motifsearchall(
    path: &str,
    motif: &str,
) -> Result<Vec<(String, usize, usize)>, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();
    let mut returnvec: Vec<(String, usize, usize)> = Vec::new();
    for i in 0..sequence.len() {
        let motifstart: usize = sequence[i].find(motif).unwrap();
        let motifend: usize = motifstart + motif.len();

        let motiftuple = (header[i].clone(), motifstart, motifend);
        returnvec.push(motiftuple);
    }
    Ok::<Vec<(String, usize, usize)>, Box<dyn Error>>(returnvec)
}
