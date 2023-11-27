use std::path::PathBuf;
use std::str::FromStr;

use reth_db::{
    mdbx::{tx::Tx, RO},
    open_db_read_only,
};
use reth_primitives::TxHash;
use reth_provider::{ProviderFactory, ReceiptProvider, TransactionsProvider};

fn main() {
    let db = open_db_read_only(&PathBuf::from("/mnt/data/eth/sepolia/reth/db"), None).unwrap();
    let spec = (*reth_primitives::SEPOLIA).clone();

    let factory: ProviderFactory<reth_db::DatabaseEnv> = ProviderFactory::new(db, spec);
    let provider: reth_provider::DatabaseProvider<Tx<RO>> = factory.provider().unwrap();

    let hash =
        TxHash::from_str("0xe2e9fd898d91d0d18e83058cdc20a2021f3c36d13f9e1c345975f24572190ad3")
            .unwrap();

    // tx works
    let tx = provider.transaction_by_hash(hash).unwrap().unwrap();
    dbg!(&tx.hash());

    // receipt fails
    let receipt = provider.receipt_by_hash(hash).unwrap().unwrap();
    dbg!(receipt);
}
