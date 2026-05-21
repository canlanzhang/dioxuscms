use crate::components::{Dog};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Dogs() -> Element {
    rsx! {
        Dog{ }
    }
}
