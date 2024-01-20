use crate::api;
use concoct::{
    hook::{use_ref, use_state},
    View,
};
use concoct_web::html;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

pub struct CrateScreen {
    name: String,
}

impl CrateScreen {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl View for CrateScreen {
    fn body(&self) -> impl concoct::Body {
        let name = self.name.clone();

        let (data, set_data) = use_state(|| None);

        use_ref(move || {
            spawn_local(async move {
                let crate_data = api::get_crate(&name).await;
                set_data(Some(Rc::new(crate_data.krate)));
            })
        });

        (
            html::h1(self.name.clone()),
            data.as_ref().map(|data| {
                (
                    html::h2(data.max_stable_version.clone()),
                    html::p(data.description.clone()),
                    html::ul((
                        data.homepage.as_ref().map(|homepage| link_item(homepage)),
                        data.repository.as_ref().map(|repo| link_item(repo)),
                    )),
                )
            }),
        )
    }
}

fn link_item(url: &str) -> impl View {
    html::li(html::a(url.to_owned()))
}
