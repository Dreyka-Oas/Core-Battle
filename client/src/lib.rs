use crate::styles::layout::LAYOUT_CSS;
use dioxus::prelude::*;

pub mod styles;

#[component]
pub fn App() -> Element {
    let mut left_collapsed = use_signal(|| false);
    let left_width = use_memo(move || if left_collapsed() { 150.0 } else { 350.0 });

    rsx! {
        style { {LAYOUT_CSS} }
        div {
            class: "app-container",
            div {
                class: "left-panel",
                style: "width: {left_width()}px",
                if !left_collapsed() {
                    div {
                        class: "left-top",
                        "Top Section"
                    }
                    div {
                        class: "left-bottom",
                        "Bottom Section"
                    }
                }
                button {
                    class: "toggle-btn",
                    onclick: move |_| left_collapsed.set(!left_collapsed()),
                    if left_collapsed() { ">" } else { "<" }
                }
            }
            div {
                class: "right-panel",
                "Main Content Area"
            }
        }
    }
}

pub fn run() {
    dioxus::launch(App);
}
