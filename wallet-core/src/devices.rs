use crate::ArcStr;
use std::{str::FromStr, sync::Arc};

use bdk::{bitcoin::util::bip32::{DerivationPath, Fingerprint}, descriptor::Descriptor};
use hwi::interface::HWIDevice;

#[derive(Clone)]
pub struct SlapsDevice {
    hwi_device: HWIDevice,
}

// TODO: is there a cheaper way to do this equality check?
// impl PartialEq for SlapsDevice {
//     fn eq(&self, other: &Self) -> bool {
//         let HWIDevice {
//             device_type: dt,
//             model: m,
//             path: p,
//             fingerprint: f,
//             ..
//         } = self.hwi_device.clone();
//         let HWIDevice {
//             device_type: o_dt,
//             model: o_m,
//             path: o_p,
//             fingerprint: o_f,
//             ..
//         } = other.hwi_device.clone();

//         dt == o_dt && m == o_m && p == o_p && f == o_f
//     }
// }

impl SlapsDevice {
    pub fn new(device: &HWIDevice) -> Self {
        Self {
            hwi_device: device.clone(),
        }
    }

    pub fn get_model(&self) -> ArcStr {
        self.hwi_device.model.clone().into()
    }

    pub fn get_fingerprint(&self) -> Fingerprint {
        self.hwi_device.fingerprint
    }

    pub fn get_path(&self) -> ArcStr {
        self.hwi_device.path.clone().into()
    }

    pub fn get_xpub(&self) -> ArcStr {
        // This bool is for testnet but I think it also works for regtest
        let using_regtest = true;
        // TODO: shouldn't there be a new() fn on DerivationPath for building this?
        let derivation_path = DerivationPath::from_str("m/84h/1h/0h").expect("Failed to get DerivationPath from string");
        self.hwi_device.get_xpub(&derivation_path, using_regtest).expect("Failed to get xpub").xpub.to_string().into()

    }

    pub fn get_device(&self) -> HWIDevice {
        self.hwi_device.clone()
    }
}

#[derive(Clone)]
pub struct SlapsDevices {
    devices: Arc<Vec<SlapsDevice>>,
}

impl SlapsDevices {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(vec![]),
        }
    }

    pub fn refresh(&mut self) {
        match HWIDevice::enumerate() {
            Ok(devices) => {
                self.devices = Arc::new(devices.iter().map(|d| SlapsDevice::new(d)).collect());
            }
            Err(err) => {
                self.devices = Arc::new(vec![]);
                eprintln!("Failed to enumerate devices: {:?}", err);
            }
        }
    }

    pub fn list_models(&self) -> Vec<ArcStr> {
        self.devices.iter().map(|d| d.get_model()).collect()
    }

    pub fn list_devices(&self) -> Vec<SlapsDevice> {
        self.devices.iter().map(|d| d.clone()).collect()
    }

    // TODO: This should return an Option?
    pub fn get_device_by_fingerprint(&self, fingerprint: Fingerprint) -> &SlapsDevice {
        let device = self
            .devices
            .iter()
            .find_map(|d| {
                if d.get_fingerprint() == fingerprint {
                    Some(d)
                } else {
                    None
                }
            })
            .expect("Couldn't find a device with that fingerprint");
        device
    }
}
