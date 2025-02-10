# phylogenomicstools
 - rust based alignmenttools for genome, metagenome, pangenome.   
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version. 

 ```
 cargo build 
 ```
 ```
 ➜  phylogenomicstools git:(main) ✗ ./target/debug/phylogenomicstools -h
 rust-phylogenomics tools

 Usage: phylogenomicstools <COMMAND>

 Commands:
  alignmerge          merge all the alignment into a single string
  alignmergeinterval  alignmerge the specific region of the alignment
  same-alignment      remove same alignment
  alignmentstats      alignmentstats
  filter-site         filter the sites with the given base
  filter-all          removes same bases across all the alignment
  filter-block        collineratiy block based alignment filtering
  alignment-view      allows for the view of the alignment
  alignment-clipview  allows for the embedded view of the clipped Alignment
  sitereplace         replace the specific sites based on the site probability
  protein-stat        estimate the protein stats
  indelreplace        indel substituter
  motif-search        search for the specific protein and the nucleotide motifs
  up-down-stream      specific part of the alignment and the upstream and the downstream of alignment
  site-alignment      extract the places of the specified base for LD analysis
  plotter             frequency plot for the alignments
  nucleotidecolour    color coded map
  proteomecolour      proteome heatmap
  help                Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
 ``` 
 
 Gaurav Sablok
