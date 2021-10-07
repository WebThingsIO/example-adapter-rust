/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::property::RandomPropertyBuilder;
use async_trait::async_trait;
use gateway_addon_rust::{
    adapter::{DeviceBuilder, PropertyBuilder},
    device::{Device, DeviceHandle},
    device_description::DeviceDescription,
};
use std::collections::HashMap;

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

    fn properties(&self) -> HashMap<String, Box<dyn PropertyBuilder>> {
        let mut properties: HashMap<String, Box<dyn PropertyBuilder>> = HashMap::new();
        properties.insert(
            "random".to_owned(),
            Box::new(RandomPropertyBuilder::new(self.update_interval)),
        );
        properties
    }

    fn id(&self) -> String {
        "random".to_owned()
    }

    fn description(&self) -> DeviceDescription {
        DeviceDescription {
            at_context: None,
            at_type: Some(vec![String::from("MultiLevelSensor")]),
            title: Some(String::from("Random")),
            description: Some(String::from("A device with a random property")),
            links: None,
            base_href: None,
            pin: None,
            credentials_required: None,
        }
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
    fn borrow_device_handle(&mut self) -> &mut DeviceHandle {
        &mut self.device_handle
    }
}
