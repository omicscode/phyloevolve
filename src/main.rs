mod align;
mod alignmentstat;
mod alignmerge;
mod args;
mod astruct;
mod samealingment;
use crate::args::CommandParse;
use crate::args::Commands;
use aligmentstat::alignmentstats;
use align::alignment;
use alignmerge::alignmerge;
use clap::Parser;
use samalignment::samealignments;

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
            let command = alignment(alignment, mergeheader.clone()).unwrap();
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
            let command = alignmerge(alignment, start, end, header).unwrap();
            println!(
                "The values after applying the score filter are:{:?}",
                command
            );
        }
        Commands::SameAlignment { alignment } => {
            let command = samealignment(alignment).unwrap();
            println!("The file has been written:{:?}", command);
        }
        Commands::AlignmentStats { alignment } => {
            let command = alignmentstats(alignment).unwrap();
            println!("The alignment stats for the given file is: {:?}", command);
        }
    }
}
