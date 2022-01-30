/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use async_trait::async_trait;
use gateway_addon_rust::{
    property, property::AtType, BuiltProperty, Property, PropertyDescription, PropertyStructure,
};
use tokio::time::{sleep, Duration};

#[property]
pub struct RandomProperty {
    update_interval: u64,
}

impl RandomProperty {
    pub fn new(update_interval: u64) -> Self {
        Self { update_interval }
    }
}

impl PropertyStructure for RandomProperty {
    type Value = u8;

    fn name(&self) -> String {
        "random".to_owned()
    }

    fn description(&self) -> PropertyDescription<Self::Value> {
        PropertyDescription::default()
            .at_type(AtType::LevelProperty)
            .title("Random")
            .description("Property with random values")
            .multiple_of(1)
            .read_only(false)
            .value(0)
            .visible(true)
    }
}

impl BuiltRandomProperty {
    pub async fn clear(&mut self) {
        log::debug!("Clearing random property");
        if let Err(err) = self.property_handle_mut().set_value(0).await {
            log::warn!("Failed to set random value: {}", err);
        }
    }
}

#[async_trait]
impl Property for BuiltRandomProperty {
    fn post_init(&mut self) {
        let mut cloned_handle = self.property_handle().clone();
        let update_interval = self.update_interval;

        tokio::spawn(async move {
            log::debug!("Updating property every {} ms", update_interval);
            loop {
                sleep(Duration::from_millis(update_interval)).await;

                if let Err(err) = cloned_handle.set_value(rand::random::<u8>()).await {
                    log::warn!("Failed to set random value: {}", err);
                }
            }
        });
    }

    async fn on_update(&mut self, value: Self::Value) -> Result<(), String> {
        let name = &self.property_handle.name;
        let old_value = &self.property_handle.description.value;

        log::debug!(
            "Value of property {} changed from {:?} to {}",
            name,
            old_value,
            value
        );

        if let Some(device) = self.property_handle.device.upgrade() {
            tokio::task::spawn(async move {
                device
                    .lock()
                    .await
                    .device_handle_mut()
                    .raise_event("value_event", None)
                    .await
                    .unwrap();
            });
            Ok(())
        } else {
            Err(format!("Value {} for {} is not a number", value, name))
        }
    }
}
