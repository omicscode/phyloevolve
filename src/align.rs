crate:: astruct::alignment;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn alignmerge(path: &str, mergeheader: &str) -> Result<(String,String), Box<dyn Error>> {
    let alignmentopen = File::open(path).expect("file not found");

    let alignmentread = BufReader::new(alignmentopen);

    let mergehead = mergeheader.to_string();

    let header: Vec<String> = Vec::new();
    let sequence: Vec<String> = Vec::new();
    let length: Vec<usize> = Vec::new();
    for i in alignmentread.lines(){
      let line = i.expect("line not found");
      if line.starts_with(">") {
           header.push(line.replace(">", ""));
      } else if !line.starts_with(">"){
          sequence.push(line)
          length.push(line.len());
      } 
    }
     for i in 0..sequence.len()-1{
         assert!(sequence[i].len() = sequence[i+1].len())
     }
    let merge:String = sequence.concat().to_string();
    println!(">{:?}\n{:?}", mergehead, sequence);
    
     Ok::<(String, String), Box<dyn Error>>("The merged results for the final sequences along with the header are".to_string(), merge)
  }
