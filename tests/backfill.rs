mod utils;

use std::sync::Arc;

use color_eyre::Result;
use iron_indexer::db::types::Address;
use iron_indexer::{
    config::{ChainConfig, Config, DbConfig, HttpConfig, RethConfig, SyncConfig},
    db::Db,
    sync::BackfillManager,
};

use reth_primitives::revm_primitives::HashMap;
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;
use utils::test_db::TestDb;

use crate::utils::test_provider::TestProvider;

#[tokio::test]
async fn backfill_covers_all_ranges() -> Result<()> {
    let config = Config {
        reth: RethConfig { db: "".into() },
        chain: ChainConfig {
            chain_id: 1,
            start_block: 0,
        },
        sync: SyncConfig {
            seed_addresses: Default::default(),
            buffer_size: 0,
            backfill_concurrency: 10,
        },
        http: HttpConfig { port: 0 },
        db: DbConfig { url: "".into() },
    };

    let (account_tx, _account_rx) = mpsc::unbounded_channel();
    let (job_tx, job_rx) = mpsc::unbounded_channel();

    let db = TestDb::connect(&config, account_tx, job_tx).await?;

    db.create_backfill_job(u8_to_addr(0x1), 10, 20).await?;
    // db.create_backfill_job(u8_to_addr(0x2), 15, 25).await?;
    // db.create_backfill_job(u8_to_addr(0x3), 20, 30).await?;

    let receiver = TestProvider::receiver();
    let mut receiver = receiver.lock().await;

    let backfill =
        BackfillManager::<TestProvider, TestDb>::start(db.clone(), &config, job_rx).await?;

    let counters = HashMap::new();
    let mut events = 0;
    // TODO: assert that we never query the same block twice
    while let Some((id, high)) = receiver.recv().await {
        counters.insert(id, high);
        dbg!(msg);
        events += 1;
    }

    sleep(std::time::Duration::from_millis(1000)).await;

    panic!();
    Ok(())
}

fn u8_to_addr(n: u8) -> Address {
    let slice = &[n; 20];
    alloy_primitives::Address::from_slice(slice).into()
}
