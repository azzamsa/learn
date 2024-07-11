use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "todos",
    version,
    about = "todos 📋 \nManage your TODOS.",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/"
)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new todo task
    Add {
        #[arg(short, long)]
        description: String,
    },
    /// Toggle the completion status of a todo item
    Toggle {
        #[arg(short, long)]
        id: String,
    },
    // Remove a todo
    Remove {
        #[arg(short, long)]
        id: String,
    },
}
