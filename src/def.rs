use clap::ValueEnum;

pub type Code = Vec<usize>;
pub type CodeSet = Vec<Code>;
pub type Hint = (usize, usize);

#[derive(Debug)]
pub struct Context {
    pub color_num: usize,
    pub pin_num: usize,
    pub duplicate: bool,
    pub policy: Policy,
    pub mode: Mode,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Policy {
    Firstpick,
    Minmax,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Mode {
    Guess,
    Mktree,
    Benchmark,
}
