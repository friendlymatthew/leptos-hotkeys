use crate::hotkeys_provider::use_hotkeys_context;
use crate::types::Hotkey;

#[cfg(not(feature = "ssr"))]
use leptos::{ev::DOMEventResponder, html::ElementDescriptor, *};

use std::collections::{HashMap, HashSet};

#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsValue;

#[cfg(not(feature = "ssr"))]
use web_sys::KeyboardEvent;

fn is_hotkey_match(hotkey: &Hotkey, pressed_keyset: &mut HashMap<String, KeyboardEvent>) -> bool {
    let mut modifiers_match = true;

    if hotkey.modifiers.ctrl {
        modifiers_match &= pressed_keyset.contains_key("control");
    }

    if hotkey.modifiers.shift {
        modifiers_match &= pressed_keyset.contains_key("shift");
    }

    if hotkey.modifiers.meta {
        modifiers_match &= pressed_keyset.contains_key("meta");
    }

    if hotkey.modifiers.alt {
        modifiers_match &= pressed_keyset.contains_key("alt");
    }

    if modifiers_match {
        let keys_match = hotkey.keys.iter().all(|key| {
            if let Some(event) = pressed_keyset.get_mut(key) {
                event.prevent_default();
                true
            } else {
                false
            }
        });

        modifiers_match && keys_match
    } else {
        false
    }
}

pub fn use_hotkeys_scoped(
    key_combination: String,
    on_triggered: Callback<()>,
    scopes: Vec<String>,
) {
    let parsed_keys: HashSet<Hotkey> = key_combination.split(',').map(Hotkey::new).collect();

    let hotkeys_context = use_hotkeys_context();
    let pressed_keys = hotkeys_context.pressed_keys;

    create_effect(move |_| {
        let active_scopes = hotkeys_context.active_scopes.get();
        let within_scope = scopes.iter().any(|scope| active_scopes.contains(scope));

        if within_scope {
            let mut pressed_keyset = pressed_keys.get();
            if let Some(matching_hotkey) = parsed_keys
                .iter()
                .find(|hotkey| is_hotkey_match(hotkey, &mut pressed_keyset))
            {
                if cfg!(feature = "debug") {
                    let message = format!("%cfiring hotkey: {}", &matching_hotkey);
                    web_sys::console::log_2(
                        &JsValue::from_str(&message),
                        &JsValue::from_str("color: #39FF14;"),
                    );
                }
                Callable::call(&on_triggered, ());
            }
        }
    });
}

pub fn use_hotkeys_ref_scoped<T>(
    node_ref: NodeRef<T>,
    key_combination: String,
    on_triggered: Callback<()>,
    scopes: Vec<String>,
) where
    T: ElementDescriptor + 'static + Clone,
{
    create_effect(move |_| {
        let parsed_keys: HashSet<Hotkey> = key_combination.split(',').map(Hotkey::new).collect();
        let scopes = scopes.clone();
        if let Some(element) = node_ref.get() {
            let keydown_closure = move |_event: KeyboardEvent| {
                let hotkeys_context = use_hotkeys_context();
                let active_scopes = hotkeys_context.active_scopes.get();
                let mut pressed_keys = hotkeys_context.pressed_keys.get();
                let within_scope = scopes.iter().any(|scope| active_scopes.contains(scope));

                if within_scope {
                    if let Some(matching_hotkey) = parsed_keys
                        .iter()
                        .find(|hotkey| is_hotkey_match(hotkey, &mut pressed_keys))
                    {
                        if cfg!(feature = "debug") {
                            let message = format!("%cfiring hotkey: {}", &matching_hotkey);
                            web_sys::console::log_2(
                                &JsValue::from_str(&message),
                                &JsValue::from_str("color: #39FF14;"),
                            );
                        }
                        Callable::call(&on_triggered, ());
                    }
                }
            };

            let _ = element.add(ev::keypress, keydown_closure);
        }
    });
}
