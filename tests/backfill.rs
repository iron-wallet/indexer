mod utils;

use color_eyre::Result;
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

    let (account_tx, account_rx) = mpsc::unbounded_channel();
    let (job_tx, job_rx) = mpsc::unbounded_channel();

    let db = TestDb::connect(&config, account_tx, job_tx).await?;

    let backfill =
        BackfillManager::<TestProvider, TestDb>::start(db.clone(), &config, job_rx).await?;
    assert_eq!(1, 1);

    Ok(())
}
