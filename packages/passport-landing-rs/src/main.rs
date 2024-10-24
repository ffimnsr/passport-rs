use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::Route;
use crate::theme::ThemeProvider;

mod components;
mod home;
mod route;
mod not_found;
mod theme;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeProvider>
            <BrowserRouter>
                <Switch<Route> render={route::switch} />
            </BrowserRouter>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
