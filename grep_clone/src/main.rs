use clap::Parser;
use std::path;

// Defining Custom Struct to hold CLI arguments.

#[derive(Parser)]
struct Cli{
    pattern: String,
    path: path::PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let args = Cli::parse();
    
    let result= std::fs::read_to_string(&args.path)?;
    println!("Result = {}", result);
    
    Ok(())
}
