use sycamore::prelude::*;
use sycamore::suspense::Suspense;

pub mod client;
mod game;
use game::create_game::CreateGame;

fn main() {
    sycamore::render(|cx| view! { cx,
        Suspense(fallback=view! { cx, "Loading..." }) {
            CreateGame {}
        }
    });
}