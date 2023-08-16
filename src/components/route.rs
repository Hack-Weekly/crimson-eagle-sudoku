use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::home::Home;
use crate::components::puzzle::Puzzle;
use crate::components::statistics::Statistics;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/puzzle")]
    Puzzle {},
    #[route("/statistics")]
    Statistics {},
}
