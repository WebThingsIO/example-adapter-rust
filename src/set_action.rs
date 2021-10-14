/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use async_trait::async_trait;
use gateway_addon_rust::{
    action::{Action, ActionHandle},
    action_description::{ActionDescription, ActionDescriptionBuilder},
};
use serde_json::json;

pub struct SetAction {}

impl SetAction {
    pub fn new() -> Self {
        SetAction {}
    }
}

#[async_trait]
impl Action for SetAction {
    type Input = u8;

    fn name(&self) -> String {
        "set".to_owned()
    }

    fn description(&self) -> ActionDescription<Self::Input> {
        ActionDescription::default()
    }

    async fn perform(
        &mut self,
        mut action_handle: ActionHandle<Self::Input>,
    ) -> Result<(), String> {
        action_handle.start().await.unwrap();
        let input = action_handle.input;
        println!("performing set action with {:?}", input);
        if let Some(device) = action_handle.device.upgrade() {
            tokio::spawn(async move {
                let mut device = device.lock().await;
                let property = device.device_handle_mut().get_property("random").unwrap();
                let mut property = property.lock().await;
                property
                    .property_handle_mut()
                    .set_value(json!(input))
                    .await
                    .unwrap();
                action_handle.finish().await.unwrap();
            });
        } else {
            eprintln!("Failed to get device ref");
            action_handle.finish().await.unwrap();
        }
        Ok(())
    }
}
