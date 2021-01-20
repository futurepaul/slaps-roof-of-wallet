use std::sync::Arc;
pub type ArcStr = Arc<str>;

mod wallet;
mod devices;
mod signer;

pub use wallet::SlapsWallet;
pub use devices::{SlapsDevices, SlapsDevice};
pub use signer::HWISigner;

