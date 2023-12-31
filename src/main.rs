use dioxus::prelude::*;

#[cfg(not(feature = "lookbook"))]
fn main() {
    use crates::Route;
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
    use crates::components::{KrateItemPreview, KratePreview};
    use lookbook::LookBook;

    #[component]
    fn Home(cx: Scope) -> Element {
        render!( h1 { "Home" } )
    }

    fn app(cx: Scope) -> Element {
        render!( LookBook { home: Home, previews: [KrateItemPreview, KratePreview] } )
    }

    dioxus_web::launch(app);
}
