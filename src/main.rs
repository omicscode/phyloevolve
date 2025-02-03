mod align;
mod alignmentstat;
mod alignmerge;
mod args;
mod astruct;
mod filterblock;
mod filtersame;
mod filtersite;
mod samealignment;
use crate::align::alignmerge;
use crate::alignmentstat::alignmentstats;
use crate::alignmerge::alignmergeall;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::filterblock::filterblockalignment;
use crate::filtersame::filtersiteall;
use crate::filtersite::filtersiteremoval;
use crate::samealignment::dealignment;
use clap::Parser;

/*
* Author Gaurav Sablok
* Date: 2025-2-2
  SLB Potsdam.
*
* a complete set of the alignment tools for the evolutionary scale and also for the graph alignments
* */

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::Alignmerge {
            alignment,
            mergeheader,
        } => {
            let command = alignmerge(alignment, mergeheader).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Alignmergeinterval {
            alignment,
            start,
            end,
            header,
        } => {
            let command = alignmergeall(alignment, start, end, header).unwrap();
            println!(
                "The values after applying the score filter are:{:?}",
                command
            );
        }
        Commands::SameAlignment { alignment } => {
            let command = dealignment(alignment).unwrap();
            println!("The file has been written:{:?}", command);
        }
        Commands::Alignmentstats { alignment } => {
            let command = alignmentstats(alignment).unwrap();
            println!("The alignment stats for the given file is: {:?}", command);
        }
        Commands::FilterSite { alignment, base } => {
            let command = filtersiteremoval(alignment, base).unwrap();
            println!(
                "The results after the filtering have been written:{:?}",
                command
            );
        }
        Commands::FilterAll { alignment } => {
            let command = filtersiteall(alignment).unwrap();
            println!("The filtered bases have been written");
        }
        Commands::FilterBlock { alignment, block } => {
            let command = filterblockalignment(alignment, block).unwrap();
            println!("The alignment block has been filtered")
        }
    }
}
