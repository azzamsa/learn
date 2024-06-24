use clap::Parser;

#[derive(Parser)]
#[command(
    name = "todos",
    version,
    about = "todos 📋 \nManage your TODOS.",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/"
)]
pub struct Opts {}
