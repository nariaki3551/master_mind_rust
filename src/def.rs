use clap::ValueEnum;

pub type Code = Vec<u8>;
pub type CodeSet = Vec<Code>;
pub type Hint = (u8, u8);

#[derive(Debug)]
pub struct Context {
    pub color_num: u8,
    pub pin_num: u8,
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
