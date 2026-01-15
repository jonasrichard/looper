use cpal::{
    SupportedStreamConfigRange,
    traits::{DeviceTrait, HostTrait},
};

#[derive(Debug, PartialEq)]
pub enum DeviceType {
    Input,
    Output,
}

#[derive(Debug)]
pub struct Device {
    pub id: String,
    pub description: String,
    pub device_type: DeviceType,
    pub configs: Vec<SupportedStreamConfigRange>,
}

pub fn get_devices() -> Vec<Device> {
    let mut devices = vec![];
    let host = cpal::default_host();

    for device in host.devices().unwrap() {
        let device = Device {
            id: device.id().unwrap().1,
            description: device.description().unwrap().name().to_string(),
            device_type: match device.supports_input() {
                true => DeviceType::Input,
                false => DeviceType::Output,
            },
            configs: device.supported_input_configs().unwrap().collect(),
        };

        println!("{device:?}");

        devices.push(device);
    }

    devices
}
