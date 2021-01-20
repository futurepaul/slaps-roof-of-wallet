use async_std::task;
use druid::{Application, ArcStr, Data, Env, EventCtx, ExtEventSink, Lens, Target};
use druid::im::{vector, Vector};
use std::{sync::Arc, time::Duration};
use wallet_core::{SlapsDevice, SlapsDevices, SlapsWallet};

use crate::selectors;

#[derive(Clone, Data)]
pub struct UIDevice {
    device: Arc<SlapsDevice>
}

impl UIDevice {
    fn new(device: &SlapsDevice) -> Self {
        Self {
            device: Arc::new(device.clone())
        }
    }

    pub fn display_model(data: &Self, _env: &Env) -> String {
        format!("Model: {}", data.device.get_model())
    }

    pub fn display_fingerprint(data: &Self, _env: &Env) -> String {
        format!("Fingerprint: {}", data.device.get_fingerprint())
    }

    pub fn display_path(data: &Self, _env: &Env) -> String {
        format!("Path: {}", data.device.get_path())
    }

    pub fn print_xpub(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        println!("xpub: {}", data.device.get_xpub());
    }

    pub fn create_wallet(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let hwi_device = data.device.clone();
        ctx.submit_command(selectors::CREATE_WALLET.with(hwi_device));
    }

}

#[derive(Clone, Copy, PartialEq, Data)]
pub enum Route {
    Setup,
    Transactions,
    Send,
    Receive
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    wallet: Arc<SlapsWallet>,
    devices: Arc<SlapsDevices>,
    ui_device_list: Vector<UIDevice>,
    pub address: ArcStr,
    pub balance: ArcStr,
    event_sink: Arc<ExtEventSink>,
    send_to_address: String,
    pub active_route: Route
}

impl AppState {
    pub fn new(sink: ExtEventSink) -> Self {
        Self {
            wallet: Arc::new(SlapsWallet::new_empty()),
            devices: Arc::new(SlapsDevices::new()),
            ui_device_list: vector![],
            address: "".into(),
            balance: "0 satoshis".into(),
            event_sink: Arc::new(sink),
            send_to_address: String::new(),
            active_route: Route::Setup
        }
    }

    pub fn refresh_devices(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        Arc::make_mut(&mut data.devices).refresh();
        let device_list: Vector<UIDevice> = data.devices.list_devices().iter().map(|d| UIDevice::new(d)).collect();
        data.ui_device_list = device_list;
    }

    pub fn set_balance(&mut self, balance: u64) {
        self.balance = format!("{} satoshis", balance).into();
    }

    pub fn get_address(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let core = data.wallet.clone();
        let sink = data.event_sink.clone();
        task::spawn(async move {
            let address: String = core.get_address().into();
            sink.submit_command(selectors::UPDATE_ADDRESS, address, Target::Auto)
                .expect("Failed to send UPDATE_ADDRESS command");
        });
    }

    pub fn copy_address(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let address = data.address.to_string();
        Application::global().clipboard().put_string(address);
    }

    pub fn paste_send_address(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if let Some(address) = Application::global().clipboard().get_string() {
            data.send_to_address = address;
        }
    }

    pub fn create_tx(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let core = data.wallet.clone();
        let address = data.send_to_address.clone();
        // let sink = data.event_sink.clone();
        let fingerprint = data.wallet.signer_fingerprint.expect("No signer fingerprint exists!");
        let device = data.devices.get_device_by_fingerprint(fingerprint).get_device();
        core.create_and_print_tx(address, device);
        //task::spawn(async move {
        //});
        
    }

    pub fn print_descriptors(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.wallet.print_descriptors();
    }

    pub fn go_to_send_route(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.active_route = Route::Send;
    }

    pub fn go_to_receive_route(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.active_route = Route::Receive;
    }

    pub fn go_to_transactions_route(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.active_route = Route::Transactions;
    }

    pub fn create_wallet_from_device(&mut self, device: Arc<SlapsDevice>) {
        self.wallet = Arc::new(SlapsWallet::new_from_hw_wallet(&device.clone()));
        self.active_route = Route::Transactions;
    }

    pub fn get_balance(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let core = data.wallet.clone();
        let sink = data.event_sink.clone();
        task::spawn(async move {
                let balance = core.get_balance();
                sink.submit_command(selectors::UPDATE_BALANCE, balance, Target::Auto)
                    .expect("Failed to send UPDATE_BALANCE command");
            }
        );
    }
}
