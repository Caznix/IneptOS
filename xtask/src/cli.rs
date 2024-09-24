use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(rename_all = "snake_case")]
pub enum Task {
    Run {
        #[command(subcommand)]
        target: Target,
    },
    Build {
        #[command(subcommand)]
        target: Target,
    },
    Clean {
        #[command(subcommand)]
        target: Target,
    },
}

#[derive(Subcommand, Debug)]
#[clap(rename_all = "snake_case")]
pub enum Target {
    X86_64,
    Aarch64,
    Riscv64,
    All,
}
