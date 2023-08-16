use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::components::route::Route;

pub fn Home(cx: Scope) -> Element {
    render! {
        div {
            class: "w-full h-screen flex flex-col justify-center items-center bg-slate-900 text-rose-50",
            h1 { class: "text-2xl font-bold", "Crimson Eagle's Sudoku" }
            div {
                class: "flex flex-col w-3/4 md:w-64 mt-32",
                HomeButton {
                    link: Route::Puzzle {},
                    label: "New Game".to_string(),
                }
                HomeButton {
                    link: Route::Statistics {},
                    label: "Continue".to_string(),
                }
                HomeButton {
                    link: Route::Statistics {},
                    label: "Statistics".to_string(),
                }
            }
        }
    }
}

#[inline_props]
fn HomeButton(cx: Scope, link: Route, label: String) -> Element {
    render! {
        Link {
            to: link.clone(),
            class: "w-full my-2 py-3 text-center font-medium rounded-lg border border-rose-600 hover:shadow-glow",
            "{label}"
        }
    }
}
