use std::error::Error;

/*

 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-3

  block size estimator in addition to the window partitioning algorithm. This partitions the seq vector into eaual patition window  


*/


pub fn splitvector(arrvec: String, blocksize: &str) -> Result<Vec<_>, Box <dyn Error>>{

   let innervec = arrvec.chars().into_iter().map(|x|String::from(x)).collect::<Vec<_>>();
    let innerblock: usize = blocksize.parse::<usize>().unwrap();
    let mut outervec: Vec<Vec<String>> = Vec:new();
   for i in 0..innervec.len()-innerblocksize {
        let start = innervec[i][0..innerblock];
  let startnext = innervec[i][start.len()..start.len()+innerblock];
        outervec.push(start);
        outervec.push(startnext);
    }
  Ok(outvec)
}
