mod bpe;
mod tokenizer;
mod utils;

use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Train {

        #[arg(short, long)]
        input: PathBuf,
        
        #[arg(short, long)]
        output: PathBuf,
        
        #[arg(short, long, default_value_t = 10000)]
        vocab_size: usize,
    },

    Tokenize {
        #[arg(short, long)]
        input: PathBuf,
        
        #[arg(short, long)]
        model: PathBuf,
        
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Train { input, output, vocab_size } => {
            println!("Training BPE model with vocabulary size {} on file {}", vocab_size, input.display());
            
            let text = fs::read_to_string(input)?;
            let mut bpe = bpe::BPE::new();
            bpe.train(&text, *vocab_size);
            
            bpe.save(output)?;
            println!("Model saved to {}", output.display());
        },
        Commands::Tokenize { input, model, output } => {
            println!("Tokenizing text using model {}", model.display());
            
            let text = fs::read_to_string(input)?;
            let bpe = bpe::BPE::load(model)?;
            
            let tokenizer = tokenizer::Tokenizer::new(bpe);
            let tokens = tokenizer.encode(&text);
            
            fs::write(output, tokens.join(" "))?;
            println!("Tokenized text saved to {}", output.display());
        }
    }

    Ok(())
}