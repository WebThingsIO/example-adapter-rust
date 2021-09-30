/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::config::Config;
use crate::device::RandomDevice;
use gateway_addon_rust::adapter::{Adapter, AdapterHandle};

pub struct ExampleAdapter {
    adapter_handle: AdapterHandle,
    config: Config,
}

impl ExampleAdapter {
    pub fn id() -> String {
        String::from("example-adapter")
    }

    pub fn name() -> String {
        String::from("Example adapter")
    }

    pub fn new(adapter_handle: AdapterHandle, config: Config) -> Self {
        ExampleAdapter {
            adapter_handle,
            config,
        }
    }

    pub async fn init(&mut self) -> Result<(), String> {
        let description = RandomDevice::build_description();
        let update_interval = self.config.update_interval;

        if let Err(err) = self
            .adapter_handle
            .add_device(description, |device_handle| {
                RandomDevice::new(device_handle, update_interval)
            })
            .await
        {
            log::error!("Failed to create device: {}", err)
        }

        Ok(())
    }
}

impl Adapter for ExampleAdapter {
    fn get_adapter_handle(&self) -> &AdapterHandle {
        &self.adapter_handle
    }
}
