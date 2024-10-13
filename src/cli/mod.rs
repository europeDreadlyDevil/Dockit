use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
pub struct CLI {
    #[command(subcommand)]
    pub(crate) command: CaptainCommand,
}

#[derive(Subcommand, Clone)]
pub enum CaptainCommand {
    Init {
        #[arg(short, long)]
        path: Option<String>
    },
    #[command(subcommand)]
    Add(Box<AddSubcommand>),
}

#[derive(Subcommand, Clone)]
pub enum AddSubcommand {
    Service {
        #[arg(short, long)]
        image: Option<String>,
        #[arg(long = "name")]
        container_name: String,
        #[arg(short, long)]
        ports: Option<Vec<String>>,
        #[arg(short, long = "env")]
        environment: Option<Vec<String>>,
        #[arg(short, long)]
        volumes: Option<Vec<String>>,
        #[arg(short, long)]
        networks: Option<Vec<String>>,
        #[arg(short, long)]
        depends_on: Option<Vec<String>>,
        #[arg(short, long)]
        command: Option<String>,
        #[arg(short, long)]
        build: Option<String>
    },
    Network {
        #[arg(short, long)]
        driver: Option<String>,
        #[arg(short, long)]
        external: Option<String>,
        #[arg(short, long)]
        name: String
    },
    Volume {
        #[arg(short, long)]
        driver: Option<String>,
        #[arg(short, long)]
        external: Option<String>,
        #[arg(short, long)]
        name: String
    },
}
