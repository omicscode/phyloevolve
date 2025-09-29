use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "phyloEVOLVE",
    version = "1.0",
    about = "rust-phylogenomicstools
    ************************************************
    Author Gaurav Sablok,
    Email: codeprog@icloud.com
    ************************************************"
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
        start: String,
        /// provide the end range
        end: String,
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
    /// filter the sites with the given base
    FilterSite {
        /// provide the path the alignment file
        alignment: String,
        /// provide the base that needs to be removed from the alignment
        base: String,
    },
    /// removes same bases across all the alignment
    FilterAll {
        /// provide the path to the alignment file
        alignment: String,
    },
    /// collineratiy block based alignment filtering
    FilterBlock {
        /// path to the alignment file
        alignment: String,
        /// provide the block size
        block: String,
    },
    /// allows for the view of the alignment
    AlignmentView {
        /// path to the alignment file
        alignment: String,
    },
    /// allows for the embedded view of the clipped Alignment
    AlignmentClipview {
        /// path to the alignment file
        alignment: String,
        /// start of the alignment
        start: usize,
        /// end of the alignment
        end: usize,
    },
    /// replace the specific sites based on the site probability
    Sitereplace {
        /// path to the alignment file
        alignment: String,
        /// path to the letter that you want to replace
        letter: String,
        /// path ot the replacement
        replacement: String,
    },
    /// estimate the protein stats
    ProteinStat {
        /// path to the protein file
        alignment: String,
    },
    /// indel substituter
    Indelreplace {
        /// path to the alignment file
        alignment: String,
        /// base sustitute for the indel
        indel: String,
    },
    /// search for the specific protein and the nucleotide motifs
    MotifSearch {
        /// path to the alignment file
        alignment: String,
        /// motif for the search
        motif: String,
    },
    /// specific part of the alignment and the upstream and the downstream of alignment
    UpDownStream {
        /// path to the alignment
        alignment: String,
        /// focal site
        focal: String,
        /// upstream alignment
        upstream: String,
        /// downstream alignment
        downstream: String,
    },
    /// extract the places of the specified base for LD analysis
    SiteAlignment {
        /// path to the alignment file
        alignment: String,
        /// path to the header
        header: String,
        /// specific base
        base: String,
    },
    /// frequency plot for the alignments
    Plotter {
        /// path to the alignment file
        alignment: String,
    },
    /// color coded map
    Nucleotidecolour {
        /// path to the alignment file
        alignment: String,
    },
    /// proteome heatmap
    Proteomecolour {
        /// path to the protein alignment
        alignment: String,
    },
}
