use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::components::sudoku;

pub fn Puzzle(cx: Scope) -> Element {
    let puzzle = sudoku::Sudoku::from(
        "003020600900305001001806400008102900700000008006708200002609500800203009005010300",
    );

    render! {
        div {
            class: "w-full h-screen flex flex-col justify-center items-center bg-slate-900 text-rose-50",
            div {
                class: "w-full md:max-w-md pl-4 pr-5 flex justify-start items-center",
                Link {
                    class: "p-2 rounded-lg border border-transparent hover:border-rose-600",
                    to: "/",
                    svg {
                        class: "w-8 h-8 fill-none stroke-current",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                        }
                        title {
                            "Menu"
                        }
                    }
                }
                h1 { class: "grow text-center text-xl font-bold mr-8", "Sudoku" }
            }
            div {
                class: "puzzle w-full md:max-w-md mt-8 flex flex-wrap justify-center",
                puzzle.grid.into_iter().enumerate().map(|(index, value)| {
                    let num: String = if value == 0 { "&nbsp;".to_string() } else { value.to_string() };
                    rsx! {
                        button {
                            key: "{index}",
                            class: "w-1/9 p-2 aspect-square border border-slate-600 hover:bg-rose-600/30 hover:border-rose-600/75",
                            dangerous_inner_html: "{num}",
                        }
                    }
                })
            }
            PuzzleMenu {}
            PuzzleKeyboard {}
        }
    }
}

fn PuzzleMenu(cx: Scope) -> Element {
    render! {
        div {
            class: "w-full md:max-w-md my-8 pl-4 pr-5 flex justify-evenly items-center",
            button {
                class: "p-2 rounded-lg border border-transparent hover:border-rose-600",
                svg {
                    class: "w-8 h-8 fill-none stroke-current",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M21 16.811c0 .864-.933 1.405-1.683.977l-7.108-4.062a1.125 1.125 0 010-1.953l7.108-4.062A1.125 1.125 0 0121 8.688v8.123zM11.25 16.811c0 .864-.933 1.405-1.683.977l-7.108-4.062a1.125 1.125 0 010-1.953L9.567 7.71a1.125 1.125 0 011.683.977v8.123z",
                    }
                    title {
                        "Undo"
                    }
                }
            }
            button {
                class: "p-2 rounded-lg border border-transparent hover:border-rose-600",
                svg {
                    class: "w-8 h-8 fill-none stroke-current",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125",
                    }
                    title {
                        "Note"
                    }
                }
            }
            button {
                class: "p-2 rounded-lg border border-transparent hover:border-rose-600",
                svg {
                    class: "w-8 h-8 fill-none stroke-current",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M12 18v-5.25m0 0a6.01 6.01 0 001.5-.189m-1.5.189a6.01 6.01 0 01-1.5-.189m3.75 7.478a12.06 12.06 0 01-4.5 0m3.75 2.383a14.406 14.406 0 01-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 10-7.517 0c.85.493 1.509 1.333 1.509 2.316V18",
                    }
                    title {
                        "Hint"
                    }
                }
            }
        }
    }
}

fn PuzzleKeyboard(cx: Scope) -> Element {
    render! {
        div {
            class: "w-full md:max-w-md pl-4 pr-5 flex flex-wrap justify-center items-center",
            for i in 1..10 {
                button {
                    class: "w-1/5 p-2 aspect-square border border-slate-600 hover:bg-rose-600/30 hover:border-rose-600/75",
                    "{i}"
                }
            }
            button {
                class: "w-1/5 p-2 aspect-square border border-slate-600 hover:bg-rose-600/30 hover:border-rose-600/75
                    flex justify-center items-center",
                svg {
                    class: "w-8 h-8 fill-none stroke-current",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M6 18L18 6M6 6l12 12",
                    }
                    title {
                        "Erase"
                    }
                }
            }
        }
    }
}
