mod utils;

use color_eyre::Result;
use iron_indexer::db::types::Address;
use iron_indexer::{
    config::{ChainConfig, Config, DbConfig, HttpConfig, RethConfig, SyncConfig},
    db::Db,
    sync::BackfillManager,
};

use tokio::sync::mpsc;

#[tokio::test]
async fn backfill_covers_all_ranges() -> Result<()> {
    let config = Config {
        reth: RethConfig {
            db: "/mnt/data/eth/sepolia/reth/db".into(),
        },
        chain: ChainConfig {
            chain_id: 11155111,
            start_block: 4500000,
        },
        sync: SyncConfig {
            seed_addresses: Default::default(),
            buffer_size: 10,
            backfill_concurrency: 10,
        },
        http: HttpConfig { port: 0 },
        db: DbConfig {
            url: "postgres://iron_indexer:iron-indexer&12345@localhost/iron_indexer_test".into(),
        },
    };

    let (account_tx, _account_rx) = mpsc::unbounded_channel();
    let (job_tx, job_rx) = mpsc::unbounded_channel();

    let db = Db::connect(&config, account_tx, job_tx).await?;

    db.create_backfill_job(u8_to_addr(0x1), 10, 20).await?;
    // db.create_backfill_job(u8_to_addr(0x2), 15, 25).await?;
    // db.create_backfill_job(u8_to_addr(0x3), 20, 30).await?;

    let _backfill = BackfillManager::start(db.clone(), &config, job_rx).await?;

    Ok(())
}

fn u8_to_addr(n: u8) -> Address {
    let slice = &[n; 20];
    alloy_primitives::Address::from_slice(slice).into()
}
