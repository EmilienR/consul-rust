#![allow(non_snake_case)]
#![allow(unused_doc_comments)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

pub mod agent;
pub mod catalog;
pub mod connect_ca;
pub mod errors;
pub mod health;
pub mod kv;
pub mod session;
pub mod status;

mod request;

use std::env;

use std::time::Duration;

use reqwest::blocking::Client as HttpClient;
use reqwest::blocking::ClientBuilder;

use errors::{Result, ResultExt};

#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config }
    }
}

#[derive(Default, Clone, Debug)]
pub struct ConfigBuilder {
    pub scheme: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub datacenter: Option<String>,
    pub namespace: Option<String>,
    pub token: Option<String>,
    pub wait_time: Option<Duration>,
    pub http_timeout: Option<Duration>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn from_full_address(full_address: &str) -> Result<ConfigBuilder> {
        let mut tmp_address = full_address;
        let mut cfg_builder = ConfigBuilder::new();
        if full_address.starts_with("http://") {
            cfg_builder = cfg_builder.with_scheme("http");
            tmp_address = &tmp_address[7..];
        } else if full_address.starts_with("https://") {
            cfg_builder = cfg_builder.with_scheme("https");
            tmp_address = &tmp_address[8..];
        } else {
            return Err(errors::Error::from(
                "No valid HTTP scheme in address".to_string(),
            ));
        }

        if let Some(colon_idx) = tmp_address.rfind(':') {
            if let Ok(port) = tmp_address[colon_idx + 1..]
                .parse::<u16>()
                .chain_err(|| format!("Invalid TCP port in {}", full_address))
            {
                cfg_builder = cfg_builder.with_port(port)
            }
            cfg_builder = cfg_builder.with_hostname(&tmp_address[..colon_idx]);
        } else {
            cfg_builder = cfg_builder.with_hostname(tmp_address);
        }

        Ok(cfg_builder)
    }

    pub fn with_scheme(mut self, scheme: &str) -> ConfigBuilder {
        self.scheme = Some(scheme.to_string());
        self
    }

    pub fn with_hostname(mut self, hostname: &str) -> ConfigBuilder {
        self.hostname = Some(hostname.to_string());
        self
    }

    pub fn with_port(mut self, port: u16) -> ConfigBuilder {
        self.port = Some(port);
        self
    }

    pub fn with_datacenter(mut self, datacenter: &str) -> ConfigBuilder {
        self.datacenter = Some(datacenter.to_string());
        self
    }

    pub fn with_namespace(mut self, namespace: &str) -> ConfigBuilder {
        self.namespace = Some(namespace.to_string());
        self
    }

    pub fn with_token(mut self, token: &str) -> ConfigBuilder {
        self.token = Some(token.to_string());
        self
    }

    pub fn with_wait_time(mut self, wait_time: Duration) -> ConfigBuilder {
        self.wait_time = Some(wait_time);
        self
    }

    pub fn with_http_timeout(mut self, http_timeout: Duration) -> ConfigBuilder {
        self.http_timeout = Some(http_timeout);
        self
    }

    pub fn build(self) -> Result<Config> {
        ClientBuilder::new()
            .timeout(self.http_timeout.unwrap_or(Duration::from_secs(10)))
            .build()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| Config {
                address: format!(
                    "{}://{}:{}",
                    self.scheme.unwrap_or("http".to_string()),
                    self.hostname.unwrap_or("localhost".to_string()),
                    self.port.unwrap_or(8500)
                ),
                datacenter: self.datacenter,
                namespace: self.namespace,
                http_client: client,
                token: self.token,
                wait_time: self.wait_time,
            })
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub address: String,
    pub datacenter: Option<String>,
    pub namespace: Option<String>,
    pub http_client: HttpClient,
    pub token: Option<String>,
    pub wait_time: Option<Duration>,
}

impl Config {
    pub fn new() -> Result<Config> {
        ClientBuilder::new()
            .build()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| Config {
                address: String::from("http://localhost:8500"),
                datacenter: None,
                namespace: None,
                http_client: client,
                token: None,
                wait_time: None,
            })
    }

    pub fn new_from_env() -> Result<Config> {
        let consul_addr = match env::var("CONSUL_HTTP_ADDR") {
            Ok(val) => {
                if val.starts_with("http") {
                    val
                } else {
                    format!("http://{}", val)
                }
            }
            Err(_e) => String::from("http://127.0.0.1:8500"),
        };
        let consul_token = env::var("CONSUL_HTTP_TOKEN").ok();
        ClientBuilder::new()
            .build()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| Config {
                address: consul_addr,
                datacenter: None,
                namespace: None,
                http_client: client,
                token: consul_token,
                wait_time: None,
            })
    }

    pub fn new_from_consul_host(
        host: &str,
        port: Option<u16>,
        token: Option<String>,
    ) -> Result<Config> {
        ClientBuilder::new()
            .build()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| Config {
                address: format!("{}:{}", host, port.unwrap_or(8500)),
                datacenter: None,
                namespace: None,
                http_client: client,
                token,
                wait_time: None,
            })
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueryOptions {
    pub datacenter: Option<String>,
    pub namespace: Option<String>,
    pub filter: Option<String>,
    pub wait_index: Option<u64>,
    pub wait_time: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct QueryMeta {
    pub last_index: Option<u64>,
    pub request_time: Duration,
}

#[derive(Clone, Debug, Default)]
pub struct WriteOptions {
    pub datacenter: Option<String>,
    pub namespace: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WriteMeta {
    pub request_time: Duration,
}
