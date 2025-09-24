use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional verbose flag
    #[arg(short, long)]
    verbose: bool,

    /// Target file(s)
    #[arg(value_hint = ValueHint::FilePath, required = true)]
    targets: Vec<String>,

    /// Output file type (positional)
    output: String,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode is ON");
    }

    println!("Target files: {:?}", cli.targets);
    println!("Output type: {}", cli.output);
}