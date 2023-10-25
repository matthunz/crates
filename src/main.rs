use dioxus::prelude::*;

#[cfg(not(feature = "lookbook"))]
fn main() {
    use crates::ui::Crates;
    use dioxus_material::{IconFont, Theme};

    fn app(cx: Scope) -> Element {
        render!(
            Theme {
                IconFont {}
                Crates {}
            }
        )
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
