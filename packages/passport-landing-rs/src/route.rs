use yew_router::prelude::*;
use yew::prelude::*;

use crate::{home::Home, not_found::NotFound};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/post-job")]
    PostJob,
    #[at("/post-gig")]
    PostGig,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::PostJob => html! { <NotFound/> },
        Route::PostGig => html! { <NotFound /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
