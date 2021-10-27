/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */

use gateway_addon_rust::{Event, EventDescription, EventHandle};
use std::time::Duration;
use tokio::time::sleep;

pub struct RandomEvent {
    update_interval: u64,
}

impl RandomEvent {
    pub fn new(update_interval: u64) -> Self {
        Self { update_interval }
    }
}

impl Event for RandomEvent {
    type Data = u8;

    fn name(&self) -> String {
        "random_event".to_owned()
    }

    fn description(&self) -> EventDescription<Self::Data> {
        EventDescription::default()
    }

    fn init(&self, event_handle: EventHandle<Self::Data>) {
        let update_interval = self.update_interval;
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
