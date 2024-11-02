use std::sync::{Arc, RwLock};

use builder::ExtensionBuilder;
use events::setup_listeners;
use futures::lock::Mutex;
use js_sys::Reflect;
use provider::{create_provider, monitor_provider, ProviderType};
use state::ExtensionState;
use tracing::{info, trace};
use url::Url;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

mod builder;
mod events;
mod provider;
mod state;
mod subscription;

const EXTENSION_PORT_NAME: &str = "frame_connect";
const CLIENT_STATUS_ALARM_KEY: &str = "check-client-status";

#[wasm_bindgen]
pub async fn initialize_extension() -> Result<JsValue, JsValue> {
    // Set up a panic hook to log errors
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Initialize tracing for logging to the console
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );

    trace!("Starting extension initialization");

    let provider = create_provider()
        .await
        .map(|client| Arc::new(RwLock::new(Some(client))))?;
    let extension = Arc::new(
        Extension::builder()
            .with_provider(provider.clone())
            .build()
            .await,
    );

    trace!("Setting up event listeners");
    setup_listeners(extension.clone(), provider.clone());
    monitor_provider(provider, extension);

    info!("Extension initialized successfully");
    Ok(true.into())
}

pub struct Extension {
    state: Arc<Mutex<ExtensionState>>,
    provider: Option<ProviderType>,
}

impl Extension {
    fn builder() -> ExtensionBuilder {
        ExtensionBuilder::new()
    }
}

fn origin_from_url(url: Option<String>) -> String {
    match url {
        Some(u) => {
            if let Ok(parsed_url) = Url::parse(&u) {
                parsed_url.origin().ascii_serialization()
            } else {
                String::new()
            }
        }
        None => String::new(),
    }
}

fn get_origin(sender: JsValue) -> String {
    let url = Reflect::get(&sender, &JsValue::from_str("url"))
        .ok()
        .and_then(|val| val.as_string());
    origin_from_url(url)
}
