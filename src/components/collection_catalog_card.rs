use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CollectionCatalogCardProps {
    pub id: String,
    pub name: String,
    pub thumbnail_image: String,
}

#[function_component(CollectionCatalogCard)]
pub fn collection_card_props(props: &CollectionCatalogCardProps) -> Html {
    let CollectionCatalogCardProps { id, name, thumbnail_image } = props;
    let href = format!("/collection/{}", id.clone());
    html! {
        <div style="
        display: grid; 
        grid-template-columns: auto 1fr; 
        gap: .5rem;
        border: solid 1px #ccc;
        border-radius: .25rem;
        overflow: hidden;
        min-height: 180px;
        ">
            <div style={format!("
            background: url({});
            width: 200px;
            background-position: center;
            background-size: cover;
            ", thumbnail_image)}></div>
            <h1 style="font-size: 1.25rem;"><a href={href}>{name.clone()}</a></h1>
        </div>

    }
}
