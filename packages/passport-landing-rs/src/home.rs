use yew::prelude::*;
use crate::components::{Footer, Header};
use crate::components::container::Container;
use crate::components::logo_cloud::LogoCloud;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="overflow-hidden">
            <Header />
            <main>
                <Container class="mt-10">
                    <LogoCloud />
                </Container>
                <div class="bg-gradient-to-b from-white from-50% to-gray-100 py-32">

                </div>
            </main>
            <Footer />
        </div>
    }
}
