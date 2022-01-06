/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::{config::Config, device::RandomDevice};
use gateway_addon_rust::{adapter, Adapter, AdapterStructure};

#[adapter]
pub struct ExampleAdapter {
    config: Config,
}

impl AdapterStructure for ExampleAdapter {
    fn id(&self) -> String {
        String::from("example-adapter")
    }

    fn name(&self) -> String {
        String::from("Example adapter")
    }
}

impl ExampleAdapter {
    pub fn new(config: Config) -> Self {
        ExampleAdapter { config }
    }
}

impl BuiltExampleAdapter {
    pub async fn init(&mut self) -> Result<(), String> {
        let update_interval = self.config.update_interval;

        if let Err(err) = self
            .adapter_handle
            .add_device(RandomDevice::new(update_interval))
            .await
        {
            log::error!("Failed to create device: {}", err)
        }

        Ok(())
    }
}

impl Adapter for BuiltExampleAdapter {}
