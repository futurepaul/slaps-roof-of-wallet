use std::fmt::Debug;

use bdk::{bitcoin::{consensus::deserialize, util::psbt::PartiallySignedTransaction}, signer::Signer};
use hwi::HWIDevice;
use bdk::bitcoin::secp256k1::{Secp256k1, All};

pub struct HWISigner {
    device: HWIDevice
}

impl HWISigner {
    pub fn new(device: HWIDevice) -> Self {
        Self {
            device
        }
    }
}

impl Debug for HWISigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HWISigner").field("device", &self.device.fingerprint.to_string()).finish()
    }
}

impl Signer for HWISigner {
    fn sign(
        &self,
        psbt: &mut bdk::bitcoin::util::psbt::PartiallySignedTransaction,
        _input_index: Option<usize>,
        _secp: &Secp256k1<All>,
    ) -> Result<(), bdk::signer::SignerError> {
        // Not sure how to sign partial?
        match self.device.sign_tx(&psbt.clone(), true) {
            Ok(hwipsbt) => { 
                *psbt = deserialize_psbt_b64(&hwipsbt.psbt);
            }
            Err(_) => { eprintln!("Failed to sign tx. I'm gonna say UserCanceled because I don't know what the actual error is");
            return Err(bdk::signer::SignerError::UserCanceled)}
        }

        Ok(())
    }

    fn sign_whole_tx(&self) -> bool {
        false 
    }
}

pub fn deserialize_psbt_b64(b64: &str) -> PartiallySignedTransaction {
    let bytes = &base64::decode(b64).expect("Failed to decode base64")[..];
    let psbt: PartiallySignedTransaction = deserialize(&bytes).expect("failed to deserialize psbt");
    psbt
}
