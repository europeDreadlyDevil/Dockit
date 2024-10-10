use crate::cli::{AddSubcommand, CaptainCommand, CLI};
use crate::compose_file::{ComposeFile, Network, Service, Volume};
use clap::Parser;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

mod cli;
mod compose_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::parse();

    match cli.command {
        CaptainCommand::Init => {
            let compose = ComposeFile::default();
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(env::current_dir()?.as_path().join("docker-compose.yml"))?
                .write_all(serde_yaml::to_string(&compose)?.as_bytes())?;
        }
        CaptainCommand::Add(command) => match *command {
            AddSubcommand::Service {
                container_name,
                image,
                ports,
                environment,
                volumes,
                networks,
                depends_on,
                command,
                build
            } => {
                let mut yaml_str = String::new();
                File::open("docker-compose.yml")?.read_to_string(&mut yaml_str)?;
                let mut compose: ComposeFile = serde_yaml::from_str(&yaml_str)?;

                let mut service = Service::new(&container_name);

                service.image(image);
                service.ports(ports);
                service.environment(environment);
                service.volumes(volumes);
                service.networks(networks);
                service.depends_on(depends_on);
                service.command(command);
                service.build(build);

                compose.add_service(&container_name, service);

                write_to_compose_file(compose)?;
            }
            AddSubcommand::Network { driver, external, name } => {
                let mut yaml_str = String::new();
                File::open("docker-compose.yml")?.read_to_string(&mut yaml_str)?;
                let mut compose: ComposeFile = serde_yaml::from_str(&yaml_str)?;

                let mut network = Network::default();

                network.driver(driver);
                network.external(external);

                compose.add_network(&name, network);

                write_to_compose_file(compose)?;
            }
            AddSubcommand::Volume { driver, external, name } => {
                let mut yaml_str = String::new();
                File::open("docker-compose.yml")?.read_to_string(&mut yaml_str)?;
                let mut compose: ComposeFile = serde_yaml::from_str(&yaml_str)?;

                let mut volume = Volume::default();

                volume.driver(driver);
                volume.external(external);

                compose.add_volume(&name, volume);

                write_to_compose_file(compose)?;
            }
        },
    }

    Ok(())
}

fn write_to_compose_file(compose_file: ComposeFile) -> Result<(), Box<dyn std::error::Error>>  {
    OpenOptions::new()
        .write(true)
        .read(true)
        .truncate(true)
        .open("docker-compose.yml")?
        .write_all(serde_yaml::to_string(&compose_file).unwrap().as_bytes())?;

    Ok(())
}
