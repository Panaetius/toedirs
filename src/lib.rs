use cfg_if::cfg_if;
pub mod app;
pub mod authentication;
pub mod config;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod models;
pub mod pages;
pub mod state;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Warn);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move || {
            view! { <App/> }
        });
    }
}}
