use bdk::database::MemoryDatabase;
use bdk::{OfflineWallet, Wallet};

pub struct SlapsCore {
    wallet: OfflineWallet<MemoryDatabase>,
}


impl SlapsCore {
    pub fn new() -> Self {
        let wallet = Self::create_wallet().expect("Failed to create wallet");
        Self { wallet }
    }

    pub fn create_wallet() -> Result<OfflineWallet<MemoryDatabase>, bdk::Error> {
        let wallet: OfflineWallet<_> = Wallet::new_offline(
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
        Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
            bitcoin::Network::Regtest,
            MemoryDatabase::default(),
        ).expect("Failed to create wallet");

        Ok(wallet)
    }

    pub fn print_address(&self) {
      println!("Address: {}", self.wallet.get_new_address().expect("Couldn't get an address TT"));
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
