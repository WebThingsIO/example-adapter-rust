/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::{
    clear_action::ClearAction, random_event::RandomEvent, random_property::RandomProperty,
    set_action::SetAction, value_event::ValueEvent,
};
use async_trait::async_trait;
use gateway_addon_rust::{
    actions, device, device::AtType, events, properties, Actions, Device, DeviceDescription,
    DeviceStructure, Events, Properties,
};

#[device]
pub struct RandomDevice {
    update_interval: u64,
}

impl RandomDevice {
    pub fn new(update_interval: u64) -> Self {
        Self { update_interval }
    }
}

impl DeviceStructure for RandomDevice {
    fn id(&self) -> String {
        "random".to_owned()
    }

    fn description(&self) -> DeviceDescription {
        DeviceDescription::default()
            .at_type(AtType::MultiLevelSensor)
            .title("random")
            .description("A device with a random property")
    }

    fn properties(&self) -> Properties {
        properties![RandomProperty::new(self.update_interval)]
    }

    fn actions(&self) -> Actions {
        actions![ClearAction::new(), SetAction::new()]
    }

    fn events(&self) -> Events {
        events![RandomEvent::new(self.update_interval), ValueEvent::new()]
    }
}

#[async_trait]
impl Device for BuiltRandomDevice {}
