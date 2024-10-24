use yew::prelude::*;
use super::hero::Hero;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <Hero />
    }
}
