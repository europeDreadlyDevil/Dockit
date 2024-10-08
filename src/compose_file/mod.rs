use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComposeFile {
    version: String,
    services: HashMap<String, Service>,
    volumes: Option<HashMap<String, Volume>>,
    networks: Option<HashMap<String, Network>>,
}

impl ComposeFile {
    pub(crate) fn add_service(&mut self, name: &str, service: Service) {
        self.services.insert(name.to_string(), service);
    }
}

impl Default for ComposeFile {
    fn default() -> Self {
        Self {
            version: "3".into(),
            services: Default::default(),
            volumes: Some(Default::default()),
            networks: Some(Default::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build: Option<BuildConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<String>,
}

impl Service {
    pub(crate) fn command(&mut self, command: Option<String>) {
        self.command = command
    }
}

impl Service {
    pub(crate) fn depends_on(&mut self, depends_on: Option<Vec<String>>) {
        self.depends_on = depends_on
    }
}

impl Service {
    pub(crate) fn networks(&mut self, networks: Option<Vec<String>>) {
        self.networks = networks
    }
}

impl Service {
    pub(crate) fn volumes(&mut self, volumes: Option<Vec<String>>) {
        self.volumes = volumes
    }
}

impl Service {
    pub(crate) fn environment(&mut self, env: Option<Vec<String>>) {
        self.environment = env
    }
}

impl Service {
    pub(crate) fn ports(&mut self, ports: Option<Vec<String>>) {
        self.ports = ports
    }
}

impl Service {
    pub(crate) fn image(&mut self, image: Option<String>) {
        self.image = image
    }
}

impl Service {
    pub(crate) fn new(name: &str) -> Service {
        Service {
            container_name: Some(name.to_string()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BuildConfig {
    context: String,
    dockerfile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Volume {
    driver: Option<String>,
    driver_opts: Option<HashMap<String, String>>,
    external: Option<bool>,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Network {
    driver: Option<String>,
    driver_opts: Option<HashMap<String, String>>,
    external: Option<bool>,
    attachable: Option<bool>,
    labels: Option<HashMap<String, String>>,
}
