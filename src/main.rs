use clap::{Parser, Subcommand};
use investec::config::{read_path,create_if_not_exist};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: FirstSubcommand,
}

#[derive(Subcommand)]
enum FirstSubcommand {
    /// Initialize project configuration
    Init {},
    /// Get Oauth Token
    Login {},
    /// Perform operations on accounts
    Accounts {
        #[clap(subcommand)]
        name: AccountOptions,
    },
}

#[derive(Subcommand, Debug)]
enum AccountOptions {
    /// Check account balance
    Balance {},
    /// List accounts
    List {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        FirstSubcommand::Init {} => {
            // todo!("Initialize .investecrc");
            let home = read_path();
            create_if_not_exist(home);
        }
        FirstSubcommand::Login {} => {
            todo!("Log in");
        }
        FirstSubcommand::Accounts { name } => match &name {
            AccountOptions::Balance {} => {
                todo!("Display account balances here");
            }
            AccountOptions::List {} => {
                todo!("List accounts here");
            }
        },
    }
}
