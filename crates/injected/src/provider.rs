use gloo_utils::format::JsValueSerdeExt;
use js_sys::{Function, Promise};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tracing::trace;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{window, CustomEvent, CustomEventInit, MessageEvent};

use crate::eip6963::{EIP6963Provider, EIP6963ProviderDetail, EIP6963ProviderInfo};

// Global thread-local storage for the provider instance
thread_local! {
    static PROVIDER_INSTANCE: RefCell<Option<Rc<RefCell<EthereumProvider>>>> = RefCell::new(None);
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct EthereumProvider {
    connected: bool,
    chain_id: String,
    accounts: Vec<String>,
    event_listeners: RefCell<HashMap<String, Vec<Function>>>,
    pending_requests: RefCell<HashMap<u64, Function>>,
}

#[wasm_bindgen]
impl EthereumProvider {
    // Constructor for EthereumProvider
    #[wasm_bindgen(constructor)]
    pub fn new() -> EthereumProvider {
        let provider = EthereumProvider {
            connected: false,
            chain_id: "".to_string(),
            accounts: vec![],
            event_listeners: RefCell::new(HashMap::new()),
            pending_requests: RefCell::new(HashMap::new()),
        };

        // Initialize the provider instance
        let provider_rc = Rc::new(RefCell::new(provider));
        PROVIDER_INSTANCE.with(|instance| {
            *instance.borrow_mut() = Some(Rc::clone(&provider_rc));
        });

        // Set up internal message handler and request listener
        provider_rc.borrow().setup_message_handler();
        EthereumProvider::listen_for_request_provider(Rc::clone(&provider_rc));

        let x = provider_rc.borrow().clone();
        x
    }

    // Public EIP-1193 `request` method, returns a Promise
    pub fn request(&self, args: JsValue) -> Result<Promise, JsValue> {
        trace!("Received request: {:?}", args);
        Ok(Promise::resolve(&JsValue::NULL))
    }

    // Public method to add an event listener
    pub fn on(&self, event: &str, listener: Function) {
        trace!("Adding listener for event: {}", event);
        let mut listeners = self.event_listeners.borrow_mut();
        listeners
            .entry(event.to_string())
            .or_insert_with(Vec::new)
            .push(listener);
    }

    // Public method to remove an event listener
    pub fn remove_listener(&self, event: &str, listener: &Function) {
        trace!("Removing listener for event: {}", event);
        let mut listeners = self.event_listeners.borrow_mut();
        if let Some(event_listeners) = listeners.get_mut(event) {
            event_listeners.retain(|l| l != listener);

            if event_listeners.is_empty() {
                listeners.remove(event);
            }
        }
    }
}

// Internal methods for EthereumProvider
impl EthereumProvider {
    // EIP-6963: Respond to `requestProvider` event with provider details
    fn listen_for_request_provider(provider_ref: Rc<RefCell<Self>>) {
        let callback = Closure::wrap(Box::new(move || {
            EthereumProvider::announce_provider(Rc::clone(&provider_ref));
        }) as Box<dyn FnMut()>);

        window()
            .expect("should have window")
            .add_event_listener_with_callback(
                "eip6963:requestProvider",
                callback.as_ref().unchecked_ref(),
            )
            .expect("could not add requestProvider event listener");

        callback.forget();
    }

    // EIP-6963: Emit the `announceProvider` `CustomEvent` with provider details
    // TODO: Consider caching the populated `CustomEvent`
    fn announce_provider(provider_ref: Rc<RefCell<Self>>) {
        let provider = provider_ref.borrow();
        let info = provider.get_info();

        // Create an EIP6963ProviderDetail with serialized info and provider as JsValue
        let detail = EIP6963ProviderDetail::new(
            JsValue::from_serde(&info).expect("Failed to serialize provider info"),
            JsValue::from(provider_ref.borrow().clone()),
        );

        let mut event_init = CustomEventInit::new();
        event_init.detail(&JsValue::from(detail));

        let custom_event =
            CustomEvent::new_with_event_init_dict("eip6963:announceProvider", &event_init)
                .expect("could not create announceProvider event");

        window()
            .expect("should have window")
            .dispatch_event(&custom_event)
            .expect("could not dispatch announceProvider event");
    }

    // Setup a message handler for handling incoming messages
    fn setup_message_handler(&self) {
        let pending_requests = self.pending_requests.clone();
        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(event) = event.dyn_into::<MessageEvent>() {
                let data = event.data();
                if let Ok(data) = data.dyn_into::<JsValue>() {
                    if let Some(payload_type) = js_sys::Reflect::get(&data, &"type".into()).ok() {
                        if payload_type == JsValue::from_str("eth:payload") {
                            if let Some(payload) =
                                js_sys::Reflect::get(&data, &"payload".into()).ok()
                            {
                                if let Some(id) = js_sys::Reflect::get(&payload, &"id".into())
                                    .ok()
                                    .and_then(|v| v.as_f64().map(|v| v as u64))
                                {
                                    if let Some(promise) = pending_requests.borrow_mut().remove(&id)
                                    {
                                        let _ = promise.call1(&JsValue::NULL, &payload);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        window()
            .expect("should have window")
            .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
            .expect("could not add event listener");

        closure.forget(); // Prevent the closure from being dropped prematurely
    }
}

// Implementing the `EIP6963Provider` trait for `EthereumProvider`
impl EIP6963Provider for EthereumProvider {
    fn get_info(&self) -> EIP6963ProviderInfo {
        EIP6963ProviderInfo {
            uuid: "00000000-0000-0000-0000-000000000000".to_string(),
            name: "Nexum".to_string(),
            icon: env!("ICON_SVG_BASE64").to_string(),
            rdns: "rs.nexum".to_string(),
        }
    }
}

