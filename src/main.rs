// src/main.rs
/*
 * Main executable for WorkflowEngine
 */

use clap::Parser;
use workflowengine::{Result, run};

#[derive(Parser)]
#[command(version, about = "WorkflowEngine - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
