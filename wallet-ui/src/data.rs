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
}

impl AppState {
    pub fn new(sink: ExtEventSink) -> Self {
        Self {
            wallet: Arc::new(SlapsWallet::new_demo()),
            devices: Arc::new(SlapsDevices::new()),
            ui_device_list: vector![],
            address: "".into(),
            balance: "0 satoshis".into(),
            event_sink: Arc::new(sink),
            send_to_address: String::new()
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
        task::spawn(async move {
            core.create_and_print_tx(address);
        });
        
    }

    pub fn start_get_balance_loop(&self) {
        let core = self.wallet.clone();
        let sink = self.event_sink.clone();
        task::spawn(async move {
            loop {
                let balance = core.get_balance();
                sink.submit_command(selectors::UPDATE_BALANCE, balance, Target::Auto)
                    .expect("Failed to send UPDATE_BALANCE command");
                task::sleep(Duration::from_secs(5)).await;
            }
        });
    }
}
