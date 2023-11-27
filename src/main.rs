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
        TxHash::from_str("0x4ebd87bd156d95ccfd256b9e63badb6b6c368685f9ef9dac4595fc7b4059150b")
            .unwrap();

    // tx works
    let tx = provider.transaction_by_hash(hash).unwrap().unwrap();
    dbg!(&tx.hash());

    // receipt fails
    let receipt = provider.receipt_by_hash(hash).unwrap().unwrap();
    dbg!(receipt);
}
