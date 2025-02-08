use crate::filesearch::read;
use colored::Colorize;
use std::error::Error;

/*
  Author Gaurav Sablok
  SLB Potsdam
  Date: 2025-2-8

 colour coded heatmaps for the nucleotide alignment.


*/

pub fn readcodecolor(path: &str) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();

    let mut vecsize: Vec<(String, usize, usize, usize, usize)> = Vec::new();

    for i in 0..sequence.len() {
        let iholda: usize = sequence[i]
            .chars()
            .map(String::from)
            .filter(|x| x == "A")
            .collect::<Vec<_>>()
            .len();

        let iholdt: usize = sequence[i]
            .chars()
            .map(String::from)
            .filter(|x| x == "T")
            .collect::<Vec<_>>()
            .len();
        let iholdg: usize = sequence[i]
            .chars()
            .map(String::from)
            .filter(|x| x == "G")
            .collect::<Vec<_>>()
            .len();
        let iholdc: usize = sequence[i]
            .chars()
            .map(String::from)
            .filter(|x| x == "C")
            .collect::<Vec<_>>()
            .len();
        vecsize.push((header[i].clone(), iholda, iholdt, iholdg, iholdc));
        for i in vecsize.iter() {
            print!("{}", i.0);
        }
    }

    let mut meda: Vec<usize> = Vec::new();
    let mut medt: Vec<usize> = Vec::new();
    let mut medc: Vec<usize> = Vec::new();
    let mut medg: Vec<usize> = Vec::new();
    for i in vecsize.iter() {
        meda.push(i.1);
        medt.push(i.2);
        medg.push(i.3);
        medc.push(i.4);
    }
    meda.sort();
    medt.sort();
    medg.sort();
    medc.sort();

    let averagea = meda[0].to_string().parse::<usize>().unwrap()
        + meda[meda.len() - 1].to_string().parse::<usize>().unwrap();
    let averaget = medt[0].to_string().parse::<usize>().unwrap()
        + medt[medt.len() - 1].to_string().parse::<usize>().unwrap();
    let averageg = medg[0].to_string().parse::<usize>().unwrap()
        + medg[medg.len() - 1].to_string().parse::<usize>().unwrap();
    let averagec = medc[0].to_string().parse::<usize>().unwrap()
        + medc[medc.len() - 1].to_string().parse::<usize>().unwrap();

    for i in vecsize.iter() {
        print!("{}", i.0);
        if i.1 > averagea {
            print!("{}", i.1.to_string().on_yellow().bold())
        } else if i.1 < averagea {
            print!("{}", i.1.to_string().on_bright_yellow().bold())
        } else if i.2 > averaget {
            print!("{}", i.2.to_string().on_red().bold())
        } else if i.2 < averaget {
            print!("{}", i.2.to_string().on_bright_red().bold())
        } else if i.3 > averageg {
            print!("{}", i.3.to_string().on_magenta().bold())
        } else if i.3 < averageg {
            print!("{}", i.3.to_string().on_bright_magenta())
        } else if i.4 > averagec {
            print!("{}", i.4.to_string().on_purple())
        } else if i.4 < averagec {
            print!("{}", i.4.to_string().on_bright_purple())
        } else {
            continue;
        }
        println!();
    }

    Ok("The colour coded results are as follows".to_string())
}
