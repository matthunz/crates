use concoct::{hook::use_state, web::html, Body, View};
use screen::CrateScreen;
use wasm_bindgen_futures::spawn_local;

mod api;

mod screen;

struct App;

impl View for App {
    fn body(&self) -> impl Body {
        CrateScreen::new(String::from("concoct"))
    }
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );

    spawn_local(concoct::run(App))
}
