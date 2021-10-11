/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::{config::Config, device::RandomDeviceBuilder};
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
        let update_interval = self.config.update_interval;

        if let Err(err) = self
            .adapter_handle
            .add_device(RandomDeviceBuilder::new(update_interval))
            .await
        {
            log::error!("Failed to create device: {}", err)
        }

        Ok(())
    }
}

impl Adapter for ExampleAdapter {
    fn adapter_handle_mut(&mut self) -> &mut AdapterHandle {
        &mut self.adapter_handle
    }
}
