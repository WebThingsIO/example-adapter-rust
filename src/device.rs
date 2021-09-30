/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::property::RandomProperty;
use async_trait::async_trait;
use gateway_addon_rust::device::{Device, DeviceHandle};
use std::collections::BTreeMap;
use webthings_gateway_ipc_types::Device as DeviceDescription;

pub struct RandomDevice {
    device_handle: DeviceHandle,
}

impl RandomDevice {
    pub fn new(mut device_handle: DeviceHandle, update_interval: u64) -> Self {
        let description = RandomProperty::build_description();

        device_handle.add_property(
            description.name.clone().unwrap(),
            description,
            |property_handle| RandomProperty::new(property_handle, update_interval),
        );

        Self { device_handle }
    }

    pub fn build_description() -> DeviceDescription {
        let mut property_descriptions = BTreeMap::new();

        let description = RandomProperty::build_description();

        property_descriptions.insert(description.name.clone().unwrap(), description);

        DeviceDescription {
            at_context: None,
            at_type: Some(vec![String::from("MultiLevelSensor")]),
            id: String::from("random"),
            title: Some(String::from("Random")),
            description: Some(String::from("A device with a random property")),
            properties: Some(property_descriptions),
            actions: None,
            events: None,
            links: None,
            base_href: None,
            pin: None,
            credentials_required: None,
        }
    }
}

#[async_trait(?Send)]
impl Device for RandomDevice {
    fn borrow_device_handle(&mut self) -> &mut DeviceHandle {
        &mut self.device_handle
    }
}
