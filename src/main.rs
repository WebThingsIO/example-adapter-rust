/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
mod adapter;
mod clear_action;
mod config;
mod device;
mod random_event;
mod random_property;
mod set_action;
mod value_event;

use as_any::Downcast;
use crate::{
    adapter::{BuiltExampleAdapter, ExampleAdapter},
    config::Config,
};
use gateway_addon_rust::{error::WebthingsError, plugin::connect};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    if let Err(err) = run().await {
        log::error!("Failed to start adapter: {}", err);
    }

    log::info!("Exiting adapter");
}

async fn run() -> Result<(), WebthingsError> {
    let mut plugin = connect("example-adapter-rust").await?;
    log::debug!("Plugin registered");

    let database = plugin.get_config_database();
    let config: Config = database.load_config()?.unwrap_or_default();

    let adapter = plugin.add_adapter(ExampleAdapter::new(config)).await?;

    let result = adapter
        .lock()
        .await
        .downcast_mut::<BuiltExampleAdapter>()
        .unwrap()
        .init()
        .await;

    if let Err(err) = result {
        plugin
            .fail(format!("Failed to initialize adapter: {}", err))
            .await?;
    }

    plugin.event_loop().await;

    Ok(())
}
