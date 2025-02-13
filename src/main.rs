mod align;
mod alignmentplot;
mod alignmentstat;
mod alignmerge;
mod args;
mod astruct;
mod colorcoded;
mod filesearch;
mod filterblock;
mod filtersame;
mod filtersite;
mod indelreplace;
mod motifsearch;
mod proteincolourcounter;
mod proteinstat;
mod samealignment;
mod sitealignment;
mod sitereplace;
mod updownstream;
mod view;
mod viewspliced;
use crate::align::alignmerge;
use crate::alignmentplot::plotter;
use crate::alignmentstat::alignmentstats;
use crate::alignmerge::alignmergeall;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::colorcoded::readcodecolor;
use crate::filterblock::filterblockalignment;
use crate::filtersame::filtersiteall;
use crate::filtersite::filtersiteremoval;
use crate::indelreplace::substitute;
use crate::motifsearch::motifsearchall;
use crate::proteincolourcounter::proteomecolor;
use crate::proteinstat::proteomestats;
use crate::samealignment::dealignment;
use crate::sitealignment::sitespecific;
use crate::sitereplace::sitereplacenuc;
use crate::updownstream::upstreamdownstream;
use crate::view::alignment_embedded_common;
use crate::viewspliced::splicedalignment;
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
            println!("The merged alignment have been written: {:?}", command);
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
            println!("The filtered bases have been written, {:?}", command);
        }
        Commands::FilterBlock { alignment, block } => {
            let command = filterblockalignment(alignment, block).unwrap();
            println!("The alignment block has been filtered, {:?}", command);
        }
        Commands::AlignmentView { alignment } => {
            let command = alignment_embedded_common(alignment);
            println!(
                "The printed alignment for the give alignment is as follows: {:?}",
                command
            );
        }
        Commands::AlignmentClipview {
            alignment,
            start,
            end,
        } => {
            let command = splicedalignment(alignment, start.clone(), end.clone());
            println!(
                "The printed alignment for the specific region are:{:?}",
                command
            );
        }
        Commands::Sitereplace {
            alignment,
            letter,
            replacement,
        } => {
            let command = sitereplacenuc(alignment, letter, replacement).unwrap();
            println!("The site have been repalced:{:?}", command)
        }
        Commands::ProteinStat { alignment } => {
            let command = proteomestats(alignment).unwrap();
            println!("The stats have been written for the file: {:?}", command);
        }
        Commands::Indelreplace { alignment, indel } => {
            let command = substitute(alignment, indel).unwrap();
            println!("The indel replacement have been written: {:?}", command);
        }
        Commands::MotifSearch { alignment, motif } => {
            let command = motifsearchall(alignment, motif).unwrap();
            println!("The search motifs has been listed:{:?}", command);
        }
        Commands::UpDownStream {
            alignment,
            focal,
            upstream,
            downstream,
        } => {
            let command = upstreamdownstream(alignment, focal, upstream, downstream).unwrap();
            println!(
                "The upstream downstream regions from the alignment have been written {:?}",
                command
            );
        }
        Commands::SiteAlignment {
            alignment,
            header,
            base,
        } => {
            let command = sitespecific(alignment, header, base).unwrap();
            println!(
                "The specific site information from all those alignments have been written:{:?}",
                command
            );
        }
        Commands::Plotter { alignment } => {
            let command = plotter(alignment).unwrap();
            println!("The alignment plots have been written:{:?}", command);
        }
        Commands::Nucleotidecolour { alignment } => {
            let command = readcodecolor(alignment).unwrap();
            println!("The colour coded alignment are as follows:{:?}", command);
        }
        Commands::Proteomecolour { alignment } => {
            let command = proteomecolor(alignment).unwrap();
            println!(
                "The color coded protein alignment protein table: {:?}",
                command
            );
        }
    }
}
