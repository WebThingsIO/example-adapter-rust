/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::random_property::RandomProperty;
use async_trait::async_trait;
use gateway_addon_rust::{action::NoInput, Action, ActionDescription, ActionHandle};

pub struct ClearAction {}

impl ClearAction {
    pub fn new() -> Self {
        ClearAction {}
    }
}

#[async_trait]
impl Action for ClearAction {
    type Input = NoInput;

    fn name(&self) -> String {
        "clear".to_owned()
    }

    fn description(&self) -> ActionDescription<Self::Input> {
        ActionDescription::default()
    }

    async fn perform(
        &mut self,
        mut action_handle: ActionHandle<Self::Input>,
    ) -> Result<(), String> {
        action_handle.start().await.unwrap();
        log::debug!("Performing clear action");
        if let Some(device) = action_handle.device.upgrade() {
            tokio::spawn(async move {
                let mut device = device.lock().await;
                let property = device.device_handle_mut().get_property("random").unwrap();
                let mut property = property.lock().await;
                let property = property
                    .as_any_mut()
                    .downcast_mut::<RandomProperty>()
                    .unwrap();
                property.clear().await;
                action_handle.finish().await.unwrap();
            });
        } else {
            log::error!("Failed to get device ref");
            action_handle.finish().await.unwrap();
        }

        Ok(())
    }
}
