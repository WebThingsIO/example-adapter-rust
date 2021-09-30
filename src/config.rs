/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default = "default_update_interval")]
    pub update_interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            update_interval: default_update_interval(),
        }
    }
}

fn default_update_interval() -> u64 {
    5000
}
