use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "alignmenttools",
    version = "1.0",
    about = "all graph and alignments tools"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// merge all the alignment into a single string.
    Alignmerge {
        /// provide the path to the alignment file
        alignment: String,
        /// provide the header to the alignment
        mergeheader: String,
    },
    /// alignmerge the specific region of the alignment
    Alignmergeinterval {
        /// provide the path to the alignment file
        alignment: String,
        /// provide the start range
        start: usize,
        /// provide the end range
        end: usize,
        /// provide the header for the merge alignment
        header: String,
    },
    /// remove same alignment
    SameAlignment {
        /// path to the alignment file
        alignment: String,
    },
    /// alignmentstats
    Alignmentstats {
        /// path  to the alignment file
        alignment: String,
    },
}
