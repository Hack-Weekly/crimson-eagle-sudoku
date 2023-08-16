#![allow(non_snake_case)]

use dioxus_fullstack::prelude::*;

mod components {
    pub mod blog;
    pub mod home;
    pub mod puzzle;
    pub mod route;
    pub mod statistics;
    pub mod sudoku;
}

use crate::components::route::Route;

fn main() {
    let config = LaunchBuilder::<FullstackRouterConfig<Route>>::router();
    #[cfg(feature = "ssr")]
    config
        .incremental(
            IncrementalRendererConfig::default()
                .invalidate_after(std::time::Duration::from_secs(120)),
        )
        .launch();

    #[cfg(not(feature = "ssr"))]
    config.launch();
}
