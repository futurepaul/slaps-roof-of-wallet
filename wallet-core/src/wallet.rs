use std::str::FromStr;

use base64;

use bdk::{bitcoin::util::bip32::{DerivationPath, ExtendedPubKey, Fingerprint}, blockchain::{noop_progress, ElectrumBlockchain}, descriptor::{Descriptor, MiniscriptKey, get_checksum}, miniscript::DescriptorPublicKey};
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use bdk::{FeeRate, TxBuilder, Wallet};

use bdk::bitcoin::{consensus::serialize, Address, Network};
use hwi::HWIDevice;

use crate::{ArcStr, SlapsDevice};

pub struct SlapsWallet {
    descriptor: ArcStr,
    change_descriptor: ArcStr,
}

fn create_descriptor(derivation_path: DerivationPath, fingerprint: Fingerprint, xpub: ExtendedPubKey, index: Option<u32>, change: bool, checksum: bool) -> Result<Descriptor<DescriptorPublicKey>, bdk::miniscript::Error> {

    let origin_prefix = derivation_path
            .to_string()
            .replace("m", &fingerprint.to_string());

    let descriptor_part = format!("[{}]{}", origin_prefix, xpub.to_string());

    let index_str = match index {
            Some(index) => index.to_string(),
            None => String::from("*"),
        };

    let inner = format!("{}/{}/{}", descriptor_part, change as u32, index_str);

    let mut descriptor = format!("wpkh({})", inner);

    if let true = checksum {
            descriptor = format!("{}#{}", descriptor, get_checksum(&descriptor).unwrap());
        };

    Descriptor::from_str(&descriptor)

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

    pub fn new_from_hw_wallet(hw_wallet: &SlapsDevice) -> Self {
        let hw_wallet = hw_wallet.get_device();
        let is_testnet = true;
        let derivation_path = DerivationPath::from_str("m/84h/1h/0h").expect("Failed to create derivation path");
        let fingerprint = hw_wallet.fingerprint;
        let xpub = hw_wallet.get_xpub(&derivation_path, is_testnet).unwrap().xpub;
        let index = None;
        let change = false;
        let checksum = false;
        let descriptor = create_descriptor(derivation_path.clone(), fingerprint, xpub, index, change, checksum).expect("Failed to create descriptor");      

        let change = true;
        let change_descriptor = create_descriptor(derivation_path.clone(), fingerprint, xpub, index, change, checksum).expect("Failed to create change descriptor");      

        Self {
            descriptor: descriptor.to_string().into(),
            change_descriptor: change_descriptor.to_string().into(),
        }
    }

    pub fn print_descriptors(&self) {
        println!("Descriptor: {}", self.descriptor);
        println!("Change Descriptor: {}", self.change_descriptor);
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
