use concoct::{hook::use_state, Body, OneOf2, View};
use concoct_web::html;
use screen::CrateScreen;
use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, InputEvent};

mod api;

mod screen;

#[derive(Clone)]
enum Screen {
    Home,
    Crate(CrateScreen),
}

struct App;

impl View for App {
    fn body(&self) -> impl Body {
        let (query, set_query) = use_state(|| String::new());
        let (screen, set_screen) = use_state(|| Screen::Home);

        (
            html::header((
                html::h5(html::a("Crates").on_click({
                    let set_screen = set_screen.clone();
                    move |_| {
                        set_screen(Screen::Home);
                    }
                })),
                html::form((
                    html::input(())
                        .on_input(move |event| {
                            let event: &InputEvent = event.unchecked_ref();
                            let target = event.target().unwrap();
                            let input: &HtmlInputElement = target.unchecked_ref();
                            set_query(input.value())
                        })
                        .kind("text")
                        .attr("value", query.to_string()),
                    html::input(()).kind("submit"),
                ))
                .class("search")
                .on_submit(move |event| {
                    event.prevent_default();
                    set_screen(Screen::Crate(CrateScreen::new(query.clone())));
                }),
            ))
            .class("header"),
            match screen {
                Screen::Home => OneOf2::A("Home"),
                Screen::Crate(crate_screen) => OneOf2::B(crate_screen.clone()),
            },
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

    spawn_local(concoct_web::run(App))
}
