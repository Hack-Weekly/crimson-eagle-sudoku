use dioxus::prelude::*;

use crate::components::sudoku;

pub fn Puzzle(cx: Scope) -> Element {
    let puzzle = sudoku::Sudoku::from(
        "003020600900305001001806400008102900700000008006708200002609500800203009005010300",
    );

    render! {
        div {
            class: "w-full h-screen flex flex-col justify-center items-center bg-slate-900 text-rose-50",
            h1 { class: "text-xl font-bold", "Sudoku" }
            div {
                class: "puzzle w-full md:max-w-md mt-8 flex flex-wrap justify-center",
                puzzle.grid.into_iter().enumerate().map(|(index, value)| {
                    let num: String = if value == 0 { "&nbsp;".to_string() } else { value.to_string() };
                    rsx! {
                        button {
                            key: "{index}",
                            class: "w-1/9 p-2 aspect-square border border-slate-600",
                            dangerous_inner_html: "{num}",
                        }
                    }
                })
            }
        }
    }
}
