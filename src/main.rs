use dioxus::prelude::*;
use dioxus_material::{use_theme, Button, Tab, TabRow, TextButton, TextField};
use lookbook::{preview, Json, LookBook};


/// Text fields let users enter text into a UI.
#[preview]
fn TextFieldPreview<'a>(
    cx: Scope,
    /// Label for the text field.
    #[lookbook(default = "Label")]
    label: &'a str,
) -> Element<'a> {
    let value = use_state(cx, || String::from("Text Field"));

    render!(
        TextField {
            label: label,
            value: value,
            onchange: move |event: FormEvent| value.set(event.data.value.clone())
        }
    )
}

#[component]
fn Home(cx: Scope) -> Element {
    render!(
        h1 { "Home" }
    )
}

fn app(cx: Scope) -> Element {
    render!(
        LookBook {
            home: Home,
            previews: [ TextFieldPreview]
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}
