use std::{process, sync::Arc};

use clap::Parser;

use todos::{
    cli::{Command, Opts},
    config::Config,
    db,
    exit_codes::ExitCode,
    output,
    todo::Repo,
};

#[tokio::main]
async fn main() {
    let result = run().await;
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            output::stderr(&format!("Error: {err:?}"));
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

async fn run() -> miette::Result<ExitCode> {
    let opts = Arc::new(Opts::parse());
    let config = Arc::new(Config::load()?);

    let pool = db::connect(&config.database).await?;
    db::migrate(&pool).await?;

    let repo = Repo::new(pool);
    match opts.cmd.as_ref() {
        Some(Command::Add { description }) => {
            repo.add(description).await?;
            output::stdout(&format!("- [ ]: {description}"));
        }
        Some(Command::Toggle { id }) => {
            let todo = repo.toggle(*id).await?;
            output::stdout(&format!("- [{}] {}", todo.done_icon(), todo.description));
        }
        Some(Command::Remove { id }) => {
            let todo = repo.remove(id.to_owned()).await?;
            output::stdout(&format!(
                "- [{}] {}. removed",
                todo.done_icon(),
                todo.description
            ));
        }
        None => {
            let todos = repo.list().await?;
            for todo in todos {
                output::stdout(&format!(
                    "- ({}) [{}]: {}",
                    todo.id,
                    todo.done_icon(),
                    &todo.description
                ));
            }
        }
    }
    Ok(ExitCode::Success)
}
