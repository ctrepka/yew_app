use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Collection {
    collection_id: String,
    name: String,
    thumbnail_image: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Collections {
    count: u16,
    results: Vec<Collection>,
    next: String,
}

#[function_component(App)]
fn app() -> Html {
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
                        Request::get("https://api.tnris.org/api/v1/collections?limit=5")
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


    collections.results.iter().map(|c| {
        let img_src = c.thumbnail_image.to_string();
        let id = format!("/collection/{}", &c.collection_id);

        html! {
            <>
                <h2>
                    <a href={id}>
                    {&c.name}
                    </a>
                </h2>
                <img src={img_src} alt="card" style="width: 400px;" />
            </>
        }
    }).collect()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
