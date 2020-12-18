use std::sync::Arc;

use druid::Selector;
use wallet_core::SlapsDevice;

use crate::data::UIDevice;

pub const UPDATE_ADDRESS: Selector<String> = Selector::new("slaps.update-address"); 
pub const UPDATE_BALANCE: Selector<u64> = Selector::new("slaps.update-balance"); 
pub const CREATE_WALLET: Selector<Arc<SlapsDevice>> = Selector::new("slaps.create-wallet"); 
