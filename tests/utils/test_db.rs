use async_trait::async_trait;
use color_eyre::Result;
use tokio::sync::mpsc::UnboundedSender;

use iron_indexer::{
    config::{ChainConfig, Config},
    db::models::BackfillJobWithId,
};

use iron_indexer::db::{
    models::{Chain, CreateTx},
    types::Address,
    Db,
};

#[derive(Clone)]
pub struct TestDb {}

#[async_trait]
impl Db for TestDb {
    async fn connect(
        _config: &Config,
        _new_accounts_tx: UnboundedSender<alloy_primitives::Address>,
        _new_job_tx: UnboundedSender<()>,
    ) -> Result<Self> {
        todo!()
    }

    async fn setup_chain(&self, _chain: &ChainConfig) -> Result<Chain> {
        todo!()
    }

    /// Updates the last known block for a chain
    async fn update_chain(&self, _id: u64, _last_known: u64) -> Result<()> {
        todo!()
    }

    async fn register(&self, _address: Address) -> Result<()> {
        todo!()
    }

    async fn create_txs(&self, _txs: Vec<CreateTx>) -> Result<()> {
        todo!()
    }

    async fn create_backfill_job(
        &self,
        _address: Address,
        _chain_id: i32,
        _low: i32,
        _high: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn get_backfill_jobs(&self) -> Result<Vec<BackfillJobWithId>> {
        todo!()
    }

    async fn rearrange_backfill_jobs(&self) -> Result<()> {
        todo!()
    }

    async fn update_job(&self, _id: i32, _high: u64) -> Result<()> {
        todo!()
    }
}
