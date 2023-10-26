use dioxus::prelude::*;

#[cfg(not(feature = "lookbook"))]
fn main() {
    use crates::ui::{KratesScreen, Route};
    use dioxus_material::{IconFont, Theme};
    use dioxus_router::prelude::Router;

    fn app(cx: Scope) -> Element {
        render!(
            Theme { 
                IconFont {}
                Router::<Route> {}
            }
        )
    }

    dioxus_web::launch(app);
}

#[cfg(feature = "lookbook")]
fn main() {
    use crates::ui::{CrateItemPreview,KratePreview};
    use lookbook::LookBook;

    #[component]
    fn Home(cx: Scope) -> Element {
        render!( h1 { "Home" } )
    }

    fn app(cx: Scope) -> Element {
        render!( LookBook { home: Home, previews: [CrateItemPreview, KratePreview] } )
    }

    dioxus_web::launch(app);
}
