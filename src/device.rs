/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::property::RandomPropertyBuilder;
use async_trait::async_trait;
use gateway_addon_rust::{
    device::{Device, DeviceBuilder, DeviceHandle},
    device_description::{AtType, DeviceDescription, DeviceDescriptionBuilder},
    property::PropertyBuilder,
};

pub struct RandomDeviceBuilder {
    update_interval: u64,
}

impl RandomDeviceBuilder {
    pub fn new(update_interval: u64) -> Self {
        Self { update_interval }
    }
}

impl DeviceBuilder<RandomDevice> for RandomDeviceBuilder {
    fn build(self, device_handle: DeviceHandle) -> RandomDevice {
        RandomDevice::new(device_handle)
    }

    fn properties(&self) -> Vec<Box<dyn PropertyBuilder>> {
        vec![Box::new(RandomPropertyBuilder::new(self.update_interval))]
    }

    fn id(&self) -> String {
        "random".to_owned()
    }

    fn description(&self) -> DeviceDescription {
        DeviceDescription::default()
            .at_type(AtType::MultiLevelSensor)
            .title("random")
            .description("A device with a random property")
    }
}

pub struct RandomDevice {
    device_handle: DeviceHandle,
}

impl RandomDevice {
    pub fn new(device_handle: DeviceHandle) -> Self {
        Self { device_handle }
    }
}

#[async_trait]
impl Device for RandomDevice {
    fn device_handle_mut(&mut self) -> &mut DeviceHandle {
        &mut self.device_handle
    }
}
