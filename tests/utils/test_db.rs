use std::ops::DerefMut;
use std::sync::Arc;

use async_trait::async_trait;
use color_eyre::Result;
use iron_indexer::db::models::BackfillJob;
use iron_indexer::rearrange::rearrange;
use tokio::sync::{mpsc::UnboundedSender, RwLock};

use iron_indexer::{
    config::{ChainConfig, Config},
    db::{
        models::{BackfillJobWithId, Chain, CreateTx},
        types::Address,
        Db,
    },
};

#[derive(Clone)]
pub struct TestDb {
    backfill_jobs: Arc<RwLock<Vec<BackfillJobWithId>>>,
}

#[async_trait]
impl Db for TestDb {
    async fn connect(
        _config: &Config,
        _new_accounts_tx: UnboundedSender<alloy_primitives::Address>,
        _new_job_tx: UnboundedSender<()>,
    ) -> Result<Self> {
        Ok(Self {
            backfill_jobs: Arc::new(RwLock::new(Vec::new())),
        })
    }

    async fn setup_chain(&self, _chain: &ChainConfig) -> Result<Chain> {
        Ok(Chain {
            chain_id: 1,
            start_block: 0,
            last_known_block: 0,
            updated_at: Default::default(),
        })
    }

    /// Updates the last known block for a chain
    async fn update_chain(&self, _id: u64, _last_known: u64) -> Result<()> {
        todo!("not needed so far")
    }

    async fn register(&self, _address: Address) -> Result<()> {
        todo!("not needed so far")
    }

    async fn create_txs(&self, _txs: Vec<CreateTx>) -> Result<()> {
        Ok(())
    }

    async fn create_backfill_job(&self, address: Address, low: i32, high: i32) -> Result<()> {
        let mut jobs = self.backfill_jobs.write().await;
        let id = jobs.len() as i32;
        jobs.push(BackfillJobWithId {
            id,
            chain_id: 1,
            addresses: vec![address],
            low,
            high,
        });
        Ok(())
    }

    async fn get_backfill_jobs(&self) -> Result<Vec<BackfillJobWithId>> {
        Ok(self.backfill_jobs.read().await.clone())
    }

    async fn rearrange_backfill_jobs(&self) -> Result<()> {
        let mut jobs = self.backfill_jobs.write().await;
        let jobs_without_id = jobs
            .iter()
            .map(|j| BackfillJob {
                addresses: j.addresses.clone(),
                chain_id: j.chain_id,
                low: j.low,
                high: j.high,
            })
            .collect::<Vec<_>>();

        let new_jobs = rearrange(&jobs_without_id, 1);

        *jobs.deref_mut() = new_jobs
            .into_iter()
            .enumerate()
            .map(|(id, j)| BackfillJobWithId {
                id: id as i32,
                addresses: j.addresses,
                chain_id: 1,
                low: j.low,
                high: j.high,
            })
            .collect();

        Ok(())
    }

    async fn update_job(&self, id: i32, high: u64) -> Result<()> {
        SENDER.send((id, high)).unwrap();
        dbg!(_id, _high);
        Ok(())
    }
}
