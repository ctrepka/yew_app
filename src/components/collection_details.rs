use crate::models::{Collection};
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CollectionProps {
    pub id: String,
}

#[function_component(CollectionDetails)]
pub fn collection_details(props: &CollectionProps) -> Html {
    let CollectionProps { id } = props;
    let url = format!("https://api.tnris.org/api/v1/collections/{}", id);
    let collection = use_state(|| Collection {
        collection_id: "".to_string(),
        name: "".to_string(),
        thumbnail_image: "".to_string(),
    });
    {
        let collection = collection.clone();
        use_effect_with_deps(
            move |_| {
                let collection = collection.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_collection: Collection = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    collection.set(fetched_collection);
                });
                || ()
            },
            (),
        );
    }

    
    html! {
        <h1>{collection.name.to_string()}</h1>
    }
}
