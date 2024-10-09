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

    pub(crate) fn add_network(&mut self, network: Network, name: &str) {
        if let Some(ref mut nets) = self.networks {
            nets.insert(name.to_string(), network);
        }
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
    pub(crate) fn new(name: &str) -> Service {
        Service {
            container_name: Some(name.to_string()),
            ..Default::default()
        }
    }
    pub(crate) fn image(&mut self, image: Option<String>) {
        self.image = image
    }
    pub(crate) fn ports(&mut self, ports: Option<Vec<String>>) {
        self.ports = ports
    }
    pub(crate) fn command(&mut self, command: Option<String>) {
        self.command = command
    }
    pub(crate) fn environment(&mut self, env: Option<Vec<String>>) {
        self.environment = env
    }
    pub(crate) fn volumes(&mut self, volumes: Option<Vec<String>>) {
        self.volumes = volumes
    }
    pub(crate) fn networks(&mut self, networks: Option<Vec<String>>) {
        self.networks = networks
    }
    pub(crate) fn depends_on(&mut self, depends_on: Option<Vec<String>>) {
        self.depends_on = depends_on
    }
    pub(crate) fn build(&mut self, dockerfile: Option<String>) {
        if dockerfile.is_some() {
            self.build = Some(BuildConfig {dockerfile, context: "./app".into()})
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BuildConfig {
    context: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Volume {
    #[serde(skip_serializing_if = "Option::is_none")]
    driver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_opts: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Volume {

}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Network {
    #[serde(skip_serializing_if = "Option::is_none")]
    driver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_opts: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<HashMap<String, String>>,
}

impl Network {
    pub(crate) fn driver(&mut self, driver: Option<String>) {
        self.driver = driver
    }

    pub(crate) fn external(&mut self, external: Option<String>) {
        if let Some(name) = external {
            let mut map = HashMap::new();
            map.insert("name".into(), name);
            self.external = Some(map)
        }
    }
}