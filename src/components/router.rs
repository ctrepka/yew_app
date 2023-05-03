use yew_router::prelude::*;
use yew::prelude::*;

use crate::components::collection_catalog::CollectionCatalog;
use crate::components::collection_details::{ CollectionDetails, CollectionProps };


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/yew_app/collection/:id")]
    Collection { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { 
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: .5rem;">
                <CollectionCatalog /> 
            </div>
            },
        Route::Collection { id } => {
            let props = CollectionProps {
                id: id,
            };

            html! {
            <CollectionDetails ..props.clone() />
            }
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}