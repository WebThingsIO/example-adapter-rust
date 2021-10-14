/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
mod adapter;
mod clear_action;
mod config;
mod device;
mod random_property;
mod set_action;

use crate::{adapter::ExampleAdapter, config::Config};
use gateway_addon_rust::{api_error::ApiError, plugin::connect};
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

async fn run() -> Result<(), ApiError> {
    let mut plugin = connect("example-adapter-rust").await?;
    log::debug!("Plugin registered");

    let database = plugin.get_config_database();
    let config: Config = database.load_config()?.unwrap_or_default();

    let adapter = plugin
        .create_adapter(
            &ExampleAdapter::id(),
            &ExampleAdapter::name(),
            |adapter_handle| ExampleAdapter::new(adapter_handle, config),
        )
        .await?;

    let result = adapter
        .lock()
        .await
        .as_any_mut()
        .downcast_mut::<ExampleAdapter>()
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
