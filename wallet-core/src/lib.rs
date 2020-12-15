use std::sync::Arc;
pub type ArcStr = Arc<str>;

mod wallet;
mod devices;

pub use wallet::SlapsWallet;
pub use devices::{SlapsDevices, SlapsDevice};

