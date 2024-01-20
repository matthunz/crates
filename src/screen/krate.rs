use crate::api;
use concoct::{
    hook::{use_effect, use_state},
    View,
};
use concoct_web::html;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Copy)]
enum Tab {
    Readme,
    Versions,
    Dependencies,
    Dependants,
}

#[derive(Clone)]
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
        let (data, set_data) = use_state(|| None);
        let (tab, set_tab) = use_state(|| Tab::Readme);

        let name = self.name.clone();
        use_effect(&self.name, move || {
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
            html::ul((
                tab_item("Readme", Tab::Readme, set_tab.clone()),
                tab_item("Versions", Tab::Versions, set_tab.clone()),
                tab_item("Dependencies", Tab::Dependencies, set_tab.clone()),
                tab_item("Dependants", Tab::Dependants, set_tab),
            ))
            .class("tabs"),
            match tab {
                Tab::Readme => "Readme",
                Tab::Versions => "Versions",
                Tab::Dependencies => "Dependencies",
                Tab::Dependants => "Dependants",
            },
        )
    }
}

fn tab_item(name: &'static str, tab: Tab, set_tab: Rc<dyn Fn(Tab)>) -> impl View {
    html::li(html::a(name).on_click(move |_| set_tab(tab))).class("tab")
}

fn link_item(url: &str) -> impl View {
    html::li(html::a(url.to_owned()))
}
