use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum PriceLevel {
    Student,
    Bediensteter,
    Gast,
}
