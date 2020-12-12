use bdk::{database::MemoryDatabase, electrum_client::Client};
use bdk::{OfflineWallet, Wallet};
use bdk::blockchain::{noop_progress, ElectrumBlockchain};

pub struct SlapsCore {
    wallet: Wallet<ElectrumBlockchain, MemoryDatabase>,
}

impl SlapsCore {
    pub fn new() -> Self {
        let wallet = Self::create_wallet().expect("Failed to create wallet");
        Self { wallet }
    }

    pub fn create_wallet() -> Result<Wallet<ElectrumBlockchain, MemoryDatabase>, bdk::Error> {
        // let client = Client::new("ssl://electrum.blockstream.info:60002", None)?;
        let client = Client::new("tcp://localhost:51401")?;
        let wallet = Wallet::new(
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
        Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
            bitcoin::Network::Regtest,
            MemoryDatabase::default(),
            ElectrumBlockchain::from(client)
        )?;

        wallet.sync(noop_progress(), None)?;


        Ok(wallet)
    }

    pub fn print_address(&self) {
      println!("Address: {}", self.wallet.get_new_address().expect("Couldn't get an address TT"));
    }

    pub fn print_balance(&self) {
      self.wallet.sync(noop_progress(), None).expect("Failed to sync");

      println!("Balance: {}", self.wallet.get_balance().expect("Failed to get balance"));
    }
}

pub fn hello_from_wallet_core() -> String {
    "Hello from wallet core".into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
