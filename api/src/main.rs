use clap::{Parser, Subcommand};
use clap_stdin::FileOrStdin;
use serde_json;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract code blocks
    Extract {
        /// Path to markdown file, or - for stdin
        #[arg(id = "path")]
        contents: FileOrStdin,

        #[arg(short, long, default_value = "text")]
        output: Output,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum Output {
    Text,
    Json,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Extract {
            contents,
            output: format,
        } => {
            let code_blocks = markdownrunner::extract(contents);
            match format {
                Output::Text => {
                    let mut output: Vec<String> = vec![];
                    for block in code_blocks {
                        output.push(block.join("\n"));
                    }
                    println!("{}", output.join("\n\n"));
                }
                Output::Json => {
                    let json = serde_json::to_value(code_blocks).unwrap();
                    println!("{}", json);
                }
            }
        }
    }
}
