#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Alignment {
    pub head: String,
    pub seq: String,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AlignmentStat {
    pub name: String,
    pub length: usize,
    pub basea: usize,
    pub baset: usize,
    pub baseg: usize,
    pub basec: usize,
    pub basen: usize,
    pub baseabsent: usize,
    pub gccontent: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Plotter {
    pub name: String,
    pub count_a: usize,
    pub count_t: usize,
    pub count_g: usize,
    pub count_c: usize,
}
