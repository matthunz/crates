use crate::{api, ui::CrateItem};

use chrono::{Utc, TimeZone};
use dioxus::prelude::*;
use dioxus_signals::use_signal;
use js_sys::Date;

#[component]
pub fn Crates(cx: Scope) -> Element {
    let crates = use_signal(cx, || None);

    use_effect(cx, (), |_| async move {
        let data = api::crates(1, 50).await.unwrap();
        crates.set(Some(data));
    });

    render!(
        ul {
            display: "flex",
            flex_direction: "column",
            gap: "20px",
            width: "100%",
            max_width: "800px",
            margin: "auto",
            border_collapse: "collapse",
            if let Some(crates) = &*crates.read() {
                render!(crates.iter().map(|krate| {
                    let date = Date::parse(&krate.updated_at);
                    let last_update = format_distance_to_now(date);
            
                    render!(CrateItem {
                        name: "{krate.name}",
                        version: "{krate.newest_version}",
                        description: "{krate.description.clone().unwrap_or_default()}",
                        total_downloads: krate.downloads,
                        recent_downloads: krate.recent_downloads,
                        last_update: "{last_update}",
                        links: &[]
                    })
                }))
            } else {
                None
            }
        }
    )
}

fn format_distance_to_now(ms: f64) -> String {
    let now = Utc::now();
    let timestamp = Utc.timestamp_millis_opt(ms as _).unwrap();

    let duration = now.signed_duration_since(timestamp);
    let abs_duration = duration.num_seconds().abs();

    if abs_duration < 60 {
        return "just now".to_string();
    } else if abs_duration < 3600 {
        let minutes = abs_duration / 60;
        if duration.num_seconds() >= 0 {
            return format!("{} minute(s) ago", minutes);
        } else {
            return format!("in {} minute(s)", minutes);
        }
    } else if abs_duration < 86400 {
        let hours = abs_duration / 3600;
        if duration.num_seconds() >= 0 {
            return format!("{} hour(s) ago", hours);
        } else {
            return format!("in {} hour(s)", hours);
        }
    } else if abs_duration < 604800 {
        let days = abs_duration / 86400;
        if duration.num_seconds() >= 0 {
            return format!("{} day(s) ago", days);
        } else {
            return format!("in {} day(s)", days);
        }
    } else {
        let years = abs_duration / 31536000; // 31536000 seconds in a year
        if duration.num_seconds() >= 0 {
            return format!("{} year(s) ago", years);
        } else {
            return format!("in {} year(s)", years);
        }
    }
}
