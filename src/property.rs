/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use async_trait::async_trait;
use gateway_addon_rust::property::{Property, PropertyHandle};
use serde_json::json;
use serde_json::value::Value;
use tokio::time::{sleep, Duration};
use webthings_gateway_ipc_types::Property as PropertyDescription;

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

    pub fn build_description() -> PropertyDescription {
        PropertyDescription {
            at_type: Some(String::from("LevelProperty")),
            name: Some(String::from("random")),
            title: Some(String::from("Random")),
            description: Some(String::from("Property with random values")),
            type_: String::from("integer"),
            unit: None,
            enum_: None,
            links: None,
            minimum: Some(0_f64),
            maximum: Some(255_f64),
            multiple_of: Some(1_f64),
            read_only: Some(false),
            value: Some(Value::from(0)),
            visible: Some(true),
        }
    }
}

#[async_trait(?Send)]
impl Property for RandomProperty {
    fn borrow_property_handle(&mut self) -> &mut PropertyHandle {
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
