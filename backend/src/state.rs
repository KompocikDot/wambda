use anyhow::Result;
use s3::{Bucket, BucketConfiguration, Region, creds::Credentials};
use sea_orm::{Database, DatabaseConnection};
use std::{env, sync::Arc};
use wasmtime::error::Context;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub blob_bucket: Arc<Bucket>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let pool = Database::connect(format!(
            "postgresql://{user}:{pass}@{hostname}:{port}/{db}",
            user = env::var("POSTGRES_USER").unwrap_or_default(),
            pass = env::var("POSTGRES_PASSWORD").unwrap_or_default(),
            hostname = env::var("POSTGRES_HOSTNAME").unwrap_or_default(),
            port = env::var("POSTGRES_PORT").unwrap_or_default(),
            db = env::var("POSTGRES_DB").unwrap_or_default(),
        ))
        .await?;

        let region = Region::Custom {
            region: "local".to_string(),
            endpoint: "http://localhost:8333".to_string(),
        };
        let credentials = Credentials::new(
            Some(""), // access-key; change later
            Some(""), // secret-key; change later
            None,     // set security_token
            None,     // session token NOTE: Should it be set?
            None,
        )?;

        Bucket::create_with_path_style(
            "binaries",
            region.clone(),
            credentials.clone(),
            BucketConfiguration::default(),
        )
        .await
        .context("could not create binaries bucket")?;

        let blob_bucket = Bucket::new("binaries", region, credentials)
            .context("could not create binaries bucket client")?
            .with_path_style();

        return Ok(Self {
            db: pool,
            blob_bucket: Arc::from(blob_bucket),
        });
    }
}
