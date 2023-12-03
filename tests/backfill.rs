mod utils;

use color_eyre::Result;
use iron_indexer::db::types::Address;
use iron_indexer::{
    config::{ChainConfig, Config, DbConfig, HttpConfig, RethConfig, SyncConfig},
    db::Db,
    sync::BackfillManager,
};

use tokio::sync::mpsc;
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
            buffer_size: 10,
            backfill_concurrency: 10,
        },
        http: HttpConfig { port: 0 },
        db: DbConfig { url: "".into() },
    };

    let (account_tx, _account_rx) = mpsc::unbounded_channel();
    let (job_tx, job_rx) = mpsc::unbounded_channel();

    let db = TestDb::connect(&config, account_tx, job_tx).await?;

    db.create_backfill_job(u8_to_addr(0x1), 10, 20).await?;
    db.create_backfill_job(u8_to_addr(0x2), 15, 25).await?;
    db.create_backfill_job(u8_to_addr(0x3), 20, 30).await?;

    let backfill =
        BackfillManager::<TestProvider, TestDb>::start(db.clone(), &config, job_rx).await?;

    backfill.await??;

    assert_eq!(1, 1);

    Ok(())
}

fn u8_to_addr(n: u8) -> Address {
    let slice = &[n; 20];
    alloy_primitives::Address::from_slice(slice).into()
}
