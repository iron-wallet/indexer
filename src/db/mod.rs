pub mod models;
mod pg_backend;
mod schema;
pub mod types;

use async_trait::async_trait;
use color_eyre::Result;
use tokio::sync::mpsc::UnboundedSender;

use crate::config::{ChainConfig, Config};
use crate::db::models::BackfillJobWithId;

use self::{
    models::{Chain, CreateTx},
    types::Address,
};

pub use pg_backend::PgBackend;

/// An abstract DB connection
/// In production, `PgBackend` is meant to be used, but the trait allows for the existance of
/// `InMemoryBackend` as well, which is useful for testing
#[async_trait]
pub trait Db: Sized + Clone + Send {
    async fn connect(
        config: &Config,
        new_accounts_tx: UnboundedSender<alloy_primitives::Address>,
        new_job_tx: UnboundedSender<()>,
    ) -> Result<Self>;

    /// Seeds the database with a chain configuration
    /// Skips if the chain already exists
    /// Returns the new or existing chain configuration
    async fn setup_chain(&self, chain: &ChainConfig) -> Result<Chain>;

    /// Updates the last known block for a chain
    async fn update_chain(&self, id: u64, last_known: u64) -> Result<()>;

    /// Register a new account
    async fn register(&self, address: Address) -> Result<()>;

    async fn create_txs(&self, txs: Vec<CreateTx>) -> Result<()>;

    async fn create_backfill_job(&self, address: Address, low: i32, high: i32) -> Result<()>;

    async fn get_backfill_jobs(&self) -> Result<Vec<BackfillJobWithId>>;

    /// Deletes all existing backfill jobs, and rearranges them for optimal I/O
    /// See `utils::rearrange` for more details
    async fn rearrange_backfill_jobs(&self) -> Result<()>;

    /// Updates the to_block for a backfill job
    async fn update_job(&self, id: i32, high: u64) -> Result<()>;
}
