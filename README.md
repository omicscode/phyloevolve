# graphtools
 - rust based graphtools for genome, metagenome, pangenome.
 - coded all the parts of this and two block collinearity algorithms for block wise comparison. 
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version. 

 ```
 cargo build 
 ```
 ```
 ➜  graphtools git:(main) ✗ ./target/debug/graphtools -h
 all graph and alignments tools

 Usage: graphtools <COMMAND>

 Commands:
  alignmerge          merge all the alignment into a single string
  alignmergeinterval  alignmerge the specific region of the alignment
  same-alignment      remove same alignment
  alignmentstats      alignmentstats
  filter-site         filter the sites with the given base
  filter-all          removes same bases across all the alignment
  filter-block        collineratiy block based alignment filtering
  help                Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
 ``` 
 Gaurav Sablok
