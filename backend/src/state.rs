use anyhow::{Context, Result};
use s3::{Bucket, BucketConfiguration, Region, creds::Credentials};
use sea_orm::{Database, DatabaseConnection};
use std::{env, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub blob_bucket: Arc<Bucket>,
    pub io_executions_bucket: Arc<Bucket>,
    pub debug_mode: bool,
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
            endpoint: "http://localhost:9000".to_string(),
        };
        let credentials = Credentials::new(
            Some(&env::var("RUSTFS_ACCESS_KEY").unwrap_or_default()), // FIXME: access-key; change later
            Some(&env::var("RUSTFS_SECRET_KEY").unwrap_or_default()), // FIXME: secret-key; change later
            None,                                                     // FIXME: set security_token
            None, // FIXME: session token NOTE: Should it be set?
            None,
        )?;

        let blob_bucket =
            Bucket::new("binaries", region.clone(), credentials.clone())?.with_path_style();

        let blob_bucket_exists = blob_bucket
            .exists()
            .await
            .context("could not check existence of a blobs bucket")?;

        if !blob_bucket_exists {
            Bucket::create_with_path_style(
                "binaries",
                region.clone(),
                credentials.clone(),
                BucketConfiguration::default(),
            )
            .await
            .context("could not create binaries bucket")?;
        }

        let executions_io_bucket =
            Bucket::new("executions-io", region.clone(), credentials.clone())?.with_path_style();

        let executions_io_bucket_exists = executions_io_bucket
            .exists()
            .await
            .context("could not check existence of a executions bucket")?;

        if !executions_io_bucket_exists {
            Bucket::create_with_path_style(
                "executions-io",
                region.clone(),
                credentials.clone(),
                BucketConfiguration::default(),
            )
            .await
            .context("could not create executions_io bucket")?;
        }

        let debug_mode = env::var_os("DEBUG").unwrap_or_default() == "1";

        return Ok(Self {
            db: pool,
            blob_bucket: Arc::from(blob_bucket),
            io_executions_bucket: Arc::from(executions_io_bucket),
            debug_mode,
        });
    }
}
