use clap::{Parser, Subcommand};

// ./target/release/mycli greet --name Venkatesh

// ./target/release/mycli add -x 7 -y 3


/// 🚀 A Simple CLI Tool
#[derive(Parser)]
#[command(name = "mycli")]
#[command(author = "Venkatesh <you@example.com>")]
#[command(version = "1.0")]
#[command(about = "Does amazing CLI things", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello to someone
    Greet {
        /// Name of the person to greet
        #[arg(short, long)]
        name: String,
    },

    NewProject{

        #[arg(short, long)]
        name:String

    },

    /// Add two numbers
    Add {
        /// First number
        #[arg(short = 'x', long)]
        x: i32,
        /// Second number
        #[arg(short = 'y', long)]
        y: i32,
    },
}

fn main() {

    let cli = Cli::parse();

    match &cli.command {

        Commands::Greet { name } => {
            println!("👋 Hello, {}!", name);
        }

        Commands::Add { x, y } => {
            println!("➕ Result: {}", x + y);
        }
        
         Commands::NewProject { name } =>{
             println!("project done {}",name);
        }
    }
}
