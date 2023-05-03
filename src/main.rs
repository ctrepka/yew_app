mod models;
mod components;

use yew::prelude::*;
use crate::components::router::Router;

#[function_component(App)]
fn app() -> Html {
    html!{
        <Router />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
