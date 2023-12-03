use color_eyre::Result;
use iron_indexer::db::models::Chain;
use iron_indexer::{config::Config, sync::Provider};
use reth_primitives::{Header, Receipt, TransactionSignedNoHash};
use std::ops::Range;

pub struct TestProvider;

impl Provider for TestProvider {
    /// Creates a new provider
    fn new(_config: &Config, _chain: &Chain) -> Result<Self> {
        todo!("not needed so far")
    }

    /// Reloads the provider
    /// This is necessary when Reth receives a new block
    fn reload(&mut self) -> Result<()> {
        todo!("not needed so far")
    }

    /// Returns the last block number
    fn last_block_number(&self) -> Result<u64> {
        todo!("not needed so far")
    }

    /// Returns a block header by number
    fn block_header(&self, _number: u64) -> Result<Option<Header>> {
        todo!("not needed so far")
    }

    /// Returns the range of transaction IDs for a block
    fn block_tx_id_ranges(&self, _number: u64) -> Result<Range<u64>> {
        todo!("not needed so far")
    }

    /// Returns a transaction by ID
    fn tx_by_id(&self, _tx_id: u64) -> Result<Option<TransactionSignedNoHash>> {
        todo!("not needed so far")
    }

    /// Returns a receipt by ID
    fn receipt_by_id(&self, _tx_id: u64) -> Result<Option<Receipt>> {
        todo!("not needed so far")
    }
}
