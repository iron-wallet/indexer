use color_eyre::Result;
use iron_indexer::db::models::Chain;
use iron_indexer::{config::Config, sync::Provider};
use lazy_static::lazy_static;
use reth_primitives::{Header, Receipt, TransactionSignedNoHash};
use std::ops::Range;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct TestProvider;

lazy_static! {
    static ref CHANNEL: (
        mpsc::UnboundedSender<()>,
        Arc<Mutex<mpsc::UnboundedReceiver<()>>>
    ) = {
        let (snd, rcv) = mpsc::unbounded_channel();
        (snd, Arc::new(Mutex::new(rcv)))
    };
}

impl TestProvider {
    pub fn receiver() -> Arc<Mutex<mpsc::UnboundedReceiver<()>>> {
        CHANNEL.1.clone()
    }
}

impl Provider for TestProvider {
    /// Creates a new provider
    fn new(_config: &Config, _chain: &Chain) -> Result<Self> {
        Ok(Self)
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
        Ok(Some(Header {
            parent_hash: Default::default(),
            ommers_hash: Default::default(),
            beneficiary: Default::default(),
            state_root: Default::default(),
            transactions_root: Default::default(),
            receipts_root: Default::default(),
            withdrawals_root: Default::default(),
            logs_bloom: Default::default(),
            difficulty: Default::default(),
            number: Default::default(),
            gas_limit: Default::default(),
            gas_used: Default::default(),
            timestamp: Default::default(),
            mix_hash: Default::default(),
            nonce: Default::default(),
            base_fee_per_gas: Default::default(),
            blob_gas_used: Default::default(),
            excess_blob_gas: Default::default(),
            parent_beacon_block_root: Default::default(),
            extra_data: Default::default(),
        }))
    }

    /// Returns the range of transaction IDs for a block
    fn block_tx_id_ranges(&self, _number: u64) -> Result<Range<u64>> {
        Ok(0..100)
    }

    /// Returns a transaction by ID
    fn tx_by_id(&self, _tx_id: u64) -> Result<Option<TransactionSignedNoHash>> {
        Ok(None)
    }

    /// Returns a receipt by ID
    fn receipt_by_id(&self, _tx_id: u64) -> Result<Option<Receipt>> {
        todo!("not needed so far")
    }
}
