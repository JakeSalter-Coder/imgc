use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional output filename flag
    #[arg(short, long)]
    output: String,

    /// Optional quality flag
    #[arg(short, long, value_parser = clap::value_parser!(i8).range(..=100))]
    quality: i8,

    /// Optional verbose flag
    #[arg(short, long)]
    verbose: bool,

    /// Target file(s)
    #[arg(value_hint = ValueHint::FilePath, required = true)]
    targets: Vec<String>,

    /// Output file type (positional)
    new: String,
}

fn main() {
    let cli = Cli::parse();

    
    println!("Output name: {}", cli.output);
    println!("Target quality: {}", cli.quality);
    if cli.verbose {
        println!("Verbose mode is ON");
    }
    println!("Target files: {:?}", cli.targets);
    println!("New type: {}", cli.new);
}