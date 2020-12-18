use std::sync::Arc;

use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};

use crate::data::AppState;
use crate::selectors::*;

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(address) = cmd.get(UPDATE_ADDRESS) {
            data.address = address.clone().into();
            Handled::Yes 
        } else if let Some(balance) = cmd.get(UPDATE_BALANCE) {
            data.set_balance(*balance);
            Handled::Yes
        } else if let Some(device) = cmd.get(CREATE_WALLET) {
            data.create_wallet_from_device(device.clone());
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}
