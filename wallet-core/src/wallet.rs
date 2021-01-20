use std::{str::FromStr, sync::Arc};

use base64;

use bdk::{ScriptType, database::MemoryDatabase, signer::{SignerId, SignerOrdering}};
use bdk::electrum_client::Client;
use bdk::{
    bitcoin::{
        consensus::deserialize,
        util::{
            bip32::{DerivationPath, ExtendedPubKey, Fingerprint},
            psbt::PartiallySignedTransaction,
        },
    },
    blockchain::{noop_progress, ElectrumBlockchain},
    descriptor::{get_checksum, Descriptor, MiniscriptKey},
    miniscript::DescriptorPublicKey,
    signer::Signer,
};
use bdk::{FeeRate, TxBuilder, Wallet};

use bdk::bitcoin::{consensus::serialize, Address, Network};
use hwi::HWIDevice;

use crate::{ArcStr, HWISigner, SlapsDevice};

pub struct SlapsWallet {
    descriptor: ArcStr,
    change_descriptor: ArcStr,
    pub signer_fingerprint: Option<Fingerprint>,
}

#[derive(Debug, Clone, PartialEq)]
struct SlapsSigner {
    pub xpub: Arc<ExtendedPubKey>,
    pub fingerprint: Arc<Fingerprint>,
    pub derivation_path: Arc<DerivationPath>,
}

impl SlapsSigner {
    pub fn new(
        fingerprint: Fingerprint,
        derivation_path: DerivationPath,
        xpub: ExtendedPubKey,
    ) -> Self {
        Self {
            fingerprint: Arc::new(fingerprint),
            derivation_path: Arc::new(derivation_path),
            xpub: Arc::new(xpub),
        }
    }
}

fn create_descriptor(
    derivation_path: DerivationPath,
    fingerprint: Fingerprint,
    xpub: ExtendedPubKey,
    index: Option<u32>,
    change: bool,
    checksum: bool,
) -> Result<Descriptor<DescriptorPublicKey>, bdk::miniscript::Error> {
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
    pub fn new_empty() -> Self {
        Self {
            descriptor: "".into(),
            change_descriptor: "".into(),
            signer_fingerprint: None,
        }
    }

    pub fn new_from_hw_wallet(hw_wallet: &SlapsDevice) -> Self {
        let hw_wallet = hw_wallet.get_device();
        let is_testnet = true;
        let derivation_path =
            DerivationPath::from_str("m/84h/1h/0h").expect("Failed to create derivation path");
        let fingerprint = hw_wallet.fingerprint;

        let descriptors = hw_wallet.get_descriptors(None, true).expect("Couldn't get descriptors from HWI");
        let descriptor = descriptors.receive[2].clone();
        let change_descriptor = descriptors.internal[2].clone();
        
        // let xpub = hw_wallet
        //     .get_xpub(&derivation_path, is_testnet)
        //     .unwrap()
        //     .xpub;
        // let index = None;
        // let change = false;
        // let checksum = false;
        // let descriptor = create_descriptor(
        //     derivation_path.clone(),
        //     fingerprint,
        //     xpub,
        //     index,
        //     change,
        //     checksum,
        // )
        // .expect("Failed to create descriptor");

        // let change = true;
        // let change_descriptor = create_descriptor(
        //     derivation_path.clone(),
        //     fingerprint,
        //     xpub,
        //     index,
        //     change,
        //     checksum,
        // )
        // .expect("Failed to create change descriptor");

        // let signer = SlapsSigner::new(fingerprint, derivation_path, xpub);

        Self {
            descriptor: descriptor.to_string().into(),
            change_descriptor: change_descriptor.to_string().into(),
            signer_fingerprint: Some(fingerprint),
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

        // TODO: actually use change descriptor
        let wallet = Wallet::new(
            descriptor,
            None,
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

    pub fn create_and_print_tx(&self, address: String, device: HWIDevice) {
        let mut wallet = self.create_wallet().expect("Failed to create wallet");
        let send_to = Address::from_str(&address).expect("Failed to parse address string");

        let (psbt, details) = wallet
            .create_tx(
                TxBuilder::with_recipients(vec![(send_to.script_pubkey(), 50_000)])
            )
            .expect("Failed to build Tx");

        println!("Transaction details: {:#?}", details);
        println!("Unsigned PSBT: {}", base64::encode(&serialize(&psbt)));

        let signer = HWISigner::new(device.clone());

        wallet.add_signer(ScriptType::Internal, device.fingerprint.into(), SignerOrdering(100), Arc::new(signer));


        // let hwi_psbt = device
        //     .sign_tx(&psbt, true)
        //     .expect("Failed to sign transaction");

        // let signed_psbt = deserialize_psbt_b64(&hwi_psbt.psbt);

        let (signed_psbt, finalized) = wallet.sign(psbt, None).expect("Failed to sign Tx");
        // let (finalized_psbt, finalized) = wallet.finalize_psbt(signed_psbt, None).expect("Failed to finalize psbt");

        // assert!(finalized, "Transaction is not finalized!");

        //println!("Signed PSBT: {}", signed_psbt);

        // wallet
        //    .broadcast(finalized_psbt.extract_tx())
        //    .expect("Failed to broadcast");
    }
}

pub fn deserialize_psbt_b64(b64: &str) -> PartiallySignedTransaction {
    let bytes = &base64::decode(b64).expect("Failed to decode bytes")[..];
    let psbt: PartiallySignedTransaction = deserialize(&bytes).expect("Failed to deserialize psbt");
    psbt
}
