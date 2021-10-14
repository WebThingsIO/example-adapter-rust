/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use async_trait::async_trait;
use gateway_addon_rust::property::{Property, PropertyBuilder, PropertyHandle};
use gateway_addon_rust::property_description::{
    AtType, PropertyDescription, PropertyDescriptionBuilder, Type,
};
use serde_json::json;
use serde_json::value::Value;
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

    fn name(&self) -> String {
        "random".to_owned()
    }

    fn description(&self) -> PropertyDescription {
        PropertyDescription::default()
            .at_type(AtType::LevelProperty)
            .title("Random")
            .description("Property with random values")
            .type_(Type::Integer)
            .maximum(255_f64)
            .minimum(0_f64)
            .multiple_of(1_f64)
            .read_only(false)
            .value(0_f64)
            .visible(true)
    }

    fn build(self: Box<Self>, property_handle: PropertyHandle) -> Self::Property {
        RandomProperty::new(property_handle, self.update_interval)
    }
}

pub struct RandomProperty {
    property_handle: PropertyHandle,
}

impl RandomProperty {
    pub fn new(property_handle: PropertyHandle, update_interval: u64) -> Self {
        let mut cloned_handle = property_handle.clone();

        tokio::spawn(async move {
            log::debug!("Updating property every {} ms", update_interval);
            loop {
                sleep(Duration::from_millis(update_interval)).await;

                if let Err(err) = cloned_handle.set_value(json!(rand::random::<u8>())).await {
                    log::warn!("Failed to set random value: {}", err);
                }
            }
        });

        RandomProperty { property_handle }
    }

    pub async fn clear(&mut self) {
        println!("Clearing random property");
        if let Err(err) = self.property_handle.set_value(json!(0)).await {
            log::warn!("Failed to set random value: {}", err);
        }
    }
}

#[async_trait]
impl Property for RandomProperty {
    fn property_handle_mut(&mut self) -> &mut PropertyHandle {
        &mut self.property_handle
    }

    async fn on_update(&mut self, value: Value) -> Result<(), String> {
        let name = &self.property_handle.name;
        let old_value = &self.property_handle.description.value;

        if let Value::Number(value) = value {
            let value = value
                .as_u64()
                .ok_or(format!("Value {} for {} is not an u64", value, name))?
                as u8;

            log::debug!(
                "Value of property {} changed from {:?} to {}",
                name,
                old_value,
                value
            );

            Ok(())
        } else {
            Err(format!("Value {} for {} is not a number", value, name))
        }
    }
}
