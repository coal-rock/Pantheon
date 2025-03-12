use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
        class: "underline bg-gray-600",
            {" hello "}
            a { "hello" }
        }
        Outlet::<Route> {}
    }
}
