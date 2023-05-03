use gloo_net::http::Request;
use yew::prelude::*;
use crate::models::{Collection, Collections};
use crate::components::collection_catalog_card::{CollectionCatalogCard, CollectionCatalogCardProps};

#[function_component(CollectionCatalog)]
pub fn collection_catalog() -> Html {
    let collections = use_state(|| Collections {
        count: 0,
        results: vec![],
        next: String::from(""),
    });
    {
        let collections = collections.clone();
        use_effect_with_deps(
            move |_| {
                let collections = collections.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_collections: Collections =
                        Request::get("https://api.tnris.org/api/v1/collections?limit=24")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    collections.set(fetched_collections);
                });
                || ()
            },
            (),
        );
    }


    collections.results.iter().map(|c: &Collection | {
        let img_src = c.thumbnail_image.to_string();

        let props: CollectionCatalogCardProps = CollectionCatalogCardProps { id: c.collection_id.clone(), name: c.name.clone(), thumbnail_image: img_src };

        html! {
            <CollectionCatalogCard ..props />
        }
    }).collect()
}