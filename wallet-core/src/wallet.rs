use std::str::FromStr;

use base64;

use bdk::blockchain::{noop_progress, ElectrumBlockchain};
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use bdk::{FeeRate, TxBuilder, Wallet};

use bdk::bitcoin::{consensus::serialize, Address, Network};

use crate::ArcStr;

pub struct SlapsWallet {
    descriptor: ArcStr,
    change_descriptor: ArcStr,
}

impl SlapsWallet {
    pub fn new_demo() -> Self {
        let descriptor: ArcStr = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)".into();
        let change_descriptor: ArcStr = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)".into();

        Self {
            descriptor,
            change_descriptor,
        }
    }

    // Create an ephemeral wallet and sync it to the blockchain
    pub fn create_wallet(&self) -> Result<Wallet<ElectrumBlockchain, MemoryDatabase>, bdk::Error> {
        let descriptor: &str = &self.descriptor.clone();
        let change_descriptor: &str = &self.change_descriptor.clone();
        let database = MemoryDatabase::default();
        let client = Client::new("tcp://localhost:51401")?;
        let wallet = Wallet::new(
            descriptor,
            Some(change_descriptor),
            Network::Regtest,
            database,
            ElectrumBlockchain::from(client),
        )?;

        wallet.sync(noop_progress(), None)?;

        Ok(wallet)
    }

    pub fn get_address(&self) -> String {
        let wallet = self.create_wallet().expect("Failed to create wallet");
        let address = wallet.get_new_address().expect("Couldn't get an address");
        address.to_string()
    }

    pub fn get_balance(&self) -> u64 {
        let wallet = self.create_wallet().expect("Failed to create wallet");
        wallet.sync(noop_progress(), None).expect("Failed to sync");

        let balance = wallet.get_balance().expect("Failed to get balance");

        balance
    }

    pub fn create_and_print_tx(&self, address: String) {
        let wallet = self.create_wallet().expect("Failed to create wallet");
        let send_to = Address::from_str(&address).expect("Failed to parse address string");

        let (psbt, details) = wallet
            .create_tx(
                TxBuilder::with_recipients(vec![(send_to.script_pubkey(), 50_000)])
                    .enable_rbf()
                    .do_not_spend_change()
                    .fee_rate(FeeRate::from_sat_per_vb(5.0)),
            )
            .expect("Failed to build Tx");

        println!("Transaction details: {:#?}", details);
        println!("Unsigned PSBT: {}", base64::encode(&serialize(&psbt)));

        let (signed_psbt, finalized) = wallet.sign(psbt, None).expect("Failed to sign Tx");

        assert!(finalized, "Transaction is not finalized!");

        println!("Signed PSBT: {}", base64::encode(&serialize(&signed_psbt)));

        wallet
            .broadcast(signed_psbt.extract_tx())
            .expect("Failed to broadcast");
    }
}
