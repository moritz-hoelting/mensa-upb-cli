use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum PriceLevel {
    Student,
    Bediensteter,
    Gast,
}

#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum Filter {
    Vegan,
    Vegetarian,
}
