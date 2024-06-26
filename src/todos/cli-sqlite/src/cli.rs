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
    /// Mark a task as completed
    Mark {
        #[arg(short, long)]
        id: i64,
    },
    /// Mark a task as todo
    Unmark {
        #[arg(short, long)]
        id: i64,
    },
    /// Remove todo
    Remove {
        #[arg(short, long)]
        id: i64,
    },
}
