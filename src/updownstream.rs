use crate::filesearch::read;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
* Author Gaurav Sablok
* Date: 2025-2-2
  SLB Potsdam.
* a complete set of the alignment tools for the evolutionary scale and also for the graph alignments
* */

#[tokio::main]
pub async fn upstreamdownstream(
    path: &str,
    focal: &str,
    upstream: &str,
    downstream: &str,
) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();

    let focalcap: usize = focal.parse::<usize>().unwrap();
    let upstreamcap: usize = upstream.parse::<usize>().unwrap();
    let downstreamcap: usize = downstream.parse::<usize>().unwrap() + 1usize;
    let mut clipped_region: Vec<String> = Vec::new();
    for i in sequence.iter() {
        let clip = i[focalcap - upstreamcap..focalcap + downstreamcap].to_string();
        clipped_region.push(clip);
    }

    let mut filewrite = File::create("clipped-regions.txt").expect("file not found");
    for i in 0..clipped_region.len() {
        writeln!(filewrite, ">{:?}\n{:?}", header[i], clipped_region[i]).expect("line not found");
    }

    Ok("The results file have been written".to_string())
}
