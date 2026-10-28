use anyhow::{Context, Result};
use log::warn;
use moka::future::Cache;
use s3::{Bucket, BucketConfiguration, Region, creds::Credentials};
use sea_orm::{Database, DatabaseConnection};
use std::{env, sync::Arc, time::Duration};
use uuid::Uuid;
use wasmtime::{Config, Engine};

/// Max total size (in bytes) of wasm binaries kept in `binaries_cache`.
pub const BINARIES_CACHE_MAX_CAPACITY: u64 = 1024 * 1024 * 1024 * 4; // 4Gb

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub blob_bucket: Arc<Bucket>,
    pub io_executions_bucket: Arc<Bucket>,
    pub debug_mode: bool,
    pub wasmtime_engine: Arc<Engine>,
    binaries_cache: Cache<Uuid, Arc<Vec<u8>>>,
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

        let mut wasmtime_config = Config::new();
        wasmtime_config.wasm_component_model_async(true);
        wasmtime_config.epoch_interruption(true);
        
        let engine = Engine::new(&wasmtime_config)?;

        // The epoch is global for the engine, so there is one ticker; every store sets its own deadline.
        let weak = engine.weak();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            while let Some(engine) = weak.upgrade() {
                interval.tick().await;
                engine.increment_epoch();
            }
        });

        return Ok(Self {
            db: pool,
            blob_bucket: Arc::from(blob_bucket),
            io_executions_bucket: Arc::from(executions_io_bucket),
            debug_mode,
            wasmtime_engine: Arc::from(engine),
            binaries_cache: Cache::builder()
                .max_capacity(BINARIES_CACHE_MAX_CAPACITY)
                .weigher(|_key, val: &Arc<Vec<u8>>| val.len() as u32)
                .build(),
        });
    }

    pub async fn get_binary(&self, id: Uuid) -> Result<Arc<Vec<u8>>> {
        if let Some(binary) = self.binaries_cache.get(&id).await {
            return Ok(binary);
        }

        let bytes = self
            .blob_bucket
            .get_object(&id.to_string())
            .await
            .context("could not read wasm binary from bucket")?
            .to_vec();
        let binary = Arc::new(bytes);

        if binary.len() as u64 > BINARIES_CACHE_MAX_CAPACITY {
            warn!(
                "binary for function {id} is {} bytes, exceeding cache capacity of {} bytes; \
                 skipping cache and serving directly from bucket",
                binary.len(),
                BINARIES_CACHE_MAX_CAPACITY
            );
            return Ok(binary);
        }

        self.binaries_cache
            .insert(id, binary.clone())
            .await;

        Ok(binary)
    }
}
