/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use async_trait::async_trait;
use gateway_addon_rust::property::{Property, PropertyBuilder, PropertyHandle};
use gateway_addon_rust::property_description::{AtType, PropertyDescription};
use tokio::time::{sleep, Duration};

pub struct RandomPropertyBuilder {
    update_interval: u64,
}

impl RandomPropertyBuilder {
    pub fn new(update_interval: u64) -> Self {
        Self { update_interval }
    }
}

impl PropertyBuilder for RandomPropertyBuilder {
    type Property = RandomProperty;
    type Value = u8;

    fn name(&self) -> String {
        "random".to_owned()
    }

    fn description(&self) -> PropertyDescription<Self::Value> {
        PropertyDescription::default()
            .at_type(AtType::LevelProperty)
            .title("Random")
            .description("Property with random values")
            .multiple_of(1_f64)
            .read_only(false)
            .value(0)
            .visible(true)
    }

    fn build(self: Box<Self>, property_handle: PropertyHandle<Self::Value>) -> Self::Property {
        RandomProperty::new(property_handle, self.update_interval)
    }
}

pub struct RandomProperty {
    property_handle: PropertyHandle<u8>,
}

impl RandomProperty {
    pub fn new(property_handle: PropertyHandle<u8>, update_interval: u64) -> Self {
        let mut cloned_handle = property_handle.clone();

        tokio::spawn(async move {
            log::debug!("Updating property every {} ms", update_interval);
            loop {
                sleep(Duration::from_millis(update_interval)).await;

                if let Err(err) = cloned_handle.set_value(rand::random::<u8>()).await {
                    log::warn!("Failed to set random value: {}", err);
                }
            }
        });

        RandomProperty { property_handle }
    }

    pub async fn clear(&mut self) {
        log::debug!("Clearing random property");
        if let Err(err) = self.property_handle.set_value(0).await {
            log::warn!("Failed to set random value: {}", err);
        }
    }
}

#[async_trait]
impl Property for RandomProperty {
    type Value = u8;

    fn property_handle_mut(&mut self) -> &mut PropertyHandle<Self::Value> {
        &mut self.property_handle
    }

    async fn on_update(&mut self, value: u8) -> Result<(), String> {
        let name = &self.property_handle.name;
        let old_value = &self.property_handle.description.value;

        log::debug!(
            "Value of property {} changed from {:?} to {}",
            name,
            old_value,
            value
        );

        if let Some(device) = self.property_handle.device.upgrade() {
            device
                .lock()
                .await
                .device_handle_mut()
                .raise_event("value_event", None)
                .await
                .unwrap();

            Ok(())
        } else {
            Err(format!("Value {} for {} is not a number", value, name))
        }
    }
}
