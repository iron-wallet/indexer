use color_eyre::Result;
use iron_indexer::db::models::Chain;
use iron_indexer::{config::Config, sync::Provider};
use reth_primitives::{Header, Receipt, TransactionSignedNoHash};
use std::ops::Range;

pub struct TestProvider;

impl Provider for TestProvider {
    /// Creates a new provider
    fn new(config: &Config, chain: &Chain) -> Result<Self> {
        todo!()
    }

    /// Reloads the provider
    /// This is necessary when Reth receives a new block
    fn reload(&mut self) -> Result<()> {
        todo!()
    }

    /// Returns the last block number
    fn last_block_number(&self) -> Result<u64> {
        todo!()
    }

    /// Returns a block header by number
    fn block_header(&self, number: u64) -> Result<Option<Header>> {
        todo!()
    }

    /// Returns the range of transaction IDs for a block
    fn block_tx_id_ranges(&self, number: u64) -> Result<Range<u64>> {
        todo!()
    }

    /// Returns a transaction by ID
    fn tx_by_id(&self, tx_id: u64) -> Result<Option<TransactionSignedNoHash>> {
        todo!()
    }

    /// Returns a receipt by ID
    fn receipt_by_id(&self, tx_id: u64) -> Result<Option<Receipt>> {
        todo!()
    }
}
