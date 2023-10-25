use dioxus::prelude::*;

#[cfg(not(feature = "lookbook"))]
fn main() {
    fn app(cx: Scope) -> Element {
        render!(h1 { "Hello World!" })
    }

    dioxus_web::launch(app);
}

#[cfg(feature = "lookbook")]
fn main() {
    use crates::ui::CrateItemPreview;
    use lookbook::LookBook;

    #[component]
    fn Home(cx: Scope) -> Element {
        render!( h1 { "Home" } )
    }

    fn app(cx: Scope) -> Element {
        render!(LookBook {
            home: Home,
            previews: [CrateItemPreview]
        })
    }

    dioxus_web::launch(app);
}
