use concoct::{hook::use_state, Body, View};
use concoct_web::html;
use screen::CrateScreen;
use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, InputEvent};

mod api;

mod screen;

struct App;

impl View for App {
    fn body(&self) -> impl Body {
        let (name, set_name) = use_state(|| None);
        let (query, set_query) = use_state(|| String::new());

        (
            html::form((
                html::input(())
                    .on_input(move |event| {
                        let event: &InputEvent = event.unchecked_ref();
                        let target = event.target().unwrap();
                        let input: &HtmlInputElement = target.unchecked_ref();
                        set_query(input.value())
                    })
                    .attr("value", query.to_string()),
                html::input(()).attr("type", "submit"),
            ))
            .on_submit(move |event| {
                event.prevent_default();
                set_name(Some(query.clone()));
            }),
            name.as_ref().map(|name| CrateScreen::new(name.to_string())),
        )
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
