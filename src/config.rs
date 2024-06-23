use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
use consulrs::kv;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Contact {
    pub name: String,
    pub url: String,
    pub email: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Base {
    pub title: String,
    pub description: String,
    pub session_exp: u32,
    pub contact: Contact,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Postgresql {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct S3 {
    pub endpoint_url: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailRabbitMQ {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub vhost: String,
    pub exchange: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Email {
    pub sender_id: String,
    pub rabbitmq: EmailRabbitMQ,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub postgresql: Postgresql,
    pub redis: Redis,
    pub s3: S3,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub base: Base,
    pub database: Database,
    pub email: Email,
}


impl Config {
    pub async fn from_consul(address: &str, key: &str) -> Result<Config, String> {

        let client = ConsulClient::new(
            ConsulClientSettingsBuilder::default()
                .address(address)
                .build()
                .unwrap()
        ).unwrap();

        let mut res = kv::read(&client, &key, None).await.unwrap();
        let raw_yaml: String = res.response.pop().unwrap().value.unwrap().try_into().unwrap();

        let config: Config = match serde_yaml::from_str(&*raw_yaml) {
            Ok(config) => config,
            Err(error) => {
                return Err(format!("Failed to parse yaml from consul -> {}", error));
            },
        };
        Ok(config)
    }
}