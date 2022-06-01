use app::{components::*, pages::*};
use leptos2::{WebComponent, DeclarativeWebApi};
use wasm_bindgen::prelude::wasm_bindgen;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn define_custom_elements() {
    // meditation
    MeditationTimer::define();
    MediaSession::define();
}