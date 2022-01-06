/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */

use gateway_addon_rust::{event, Event, EventDescription, EventStructure};
use std::time::Duration;
use tokio::time::sleep;

#[event]
pub struct RandomEvent {
    update_interval: u64,
}

impl RandomEvent {
    pub fn new(update_interval: u64) -> Self {
        Self { update_interval }
    }
}

impl EventStructure for RandomEvent {
    type Data = u8;

    fn name(&self) -> String {
        "random_event".to_owned()
    }

    fn description(&self) -> EventDescription<Self::Data> {
        EventDescription::default()
    }
}

impl Event for BuiltRandomEvent {
    fn post_init(&mut self) {
        let update_interval = self.update_interval;
        let event_handle = self.event_handle.clone();
        tokio::spawn(async move {
            log::debug!("Raising event every {} ms", update_interval);
            sleep(Duration::from_millis(update_interval / 2)).await;

            loop {
                sleep(Duration::from_millis(update_interval)).await;

                if let Err(err) = event_handle.raise(rand::random()).await {
                    log::warn!("Failed to raise random event: {}", err);
                }
            }
        });
    }
}
