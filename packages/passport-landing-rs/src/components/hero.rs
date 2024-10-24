
use yew::prelude::*;
use super::button::{Button, ButtonVariant};
use super::container::Container;
use super::gradient::Gradient;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <div class="relative">
            <Gradient class="absolute inset-2 bottom-0 rounded-4xl ring-1 ring-inset ring-black/5" />
            <Container class="relative">
                // <Navbar
                //     banner={
                //         <a
                //             href="/blog/radiant-raises-100m-series-a-from-tailwind-ventures"
                //             class="flex items-center gap-1 rounded-full bg-fuchsia-950/35 px-3 py-0.5 text-sm/6 font-medium text-white data-[hover]:bg-fuchsia-950/30"
                //         >
                //             Radiant raises $100M Series A from Tailwind Ventures
                //             <ChevronRightIcon className="size-4" />
                //         </a>
                //     }
                // />
                <div class="pb-24 pt-16 sm:pb-32 sm:pt-24 md:pb-48 md:pt-32">
                    <h1 class="font-display text-balance text-6xl/[0.9] font-medium tracking-tight text-gray-950 sm:text-8xl/[0.8] md:text-9xl/[0.8]">
                        {"Close every deal."}
                    </h1>
                    <p class="mt-8 max-w-lg text-xl/7 font-medium text-gray-950/75 sm:text-2xl/8">
                        {"Radiant helps you sell more by revealing sensitive information about your customers."}
                    </p>
                    <div class="mt-12 flex flex-col gap-x-6 gap-y-4 sm:flex-row">
                        <Button href="#">{"Get started"}</Button>
                        <Button variant={ButtonVariant::Secondary} href="/pricing">
                            {"See pricing"}
                        </Button>
                    </div>
                </div>
            </Container>
        </div>
    }
}
