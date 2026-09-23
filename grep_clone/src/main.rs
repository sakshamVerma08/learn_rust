use clap::Parser;
use anyhow::{Context,Result};
use std::path;

// Defining Custom Struct to hold CLI arguments.

#[derive(Parser)]
struct Cli{
    pattern: String,
    path: path::PathBuf,
}

fn main() -> Result<()> {
    
    let args = Cli::parse();
    
    // Grabbing the text of the file into a Result Enum.
    let result = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read the file {}", args.path.display() ))?;

    
    for line in result.lines() {

        if line.contains(&args.pattern){
            println!("{}", line);
        }

    }

    Ok(())
}
